#![allow(unsafe_code)]

use std::ffi::c_void;
use std::sync::Arc;

use amml_game::EntryId;
use amml_msvc::MsvcHeap;
use amml_sdk::{AmmlMod, Api, ModDirs, SdkError};
use crossbeam_channel::bounded;
use heads_core::config::HeadsSettings;
use heads_core::services::FaceStore;

use crossbeam_channel::Sender;
use heads_core::services::GlyphPage;
use heads_core::structs::FaceUpdate;

use crate::detours::{char_width, client_update, draw_icon, exit_level, gui_message, player_list, player_skin, render};
use crate::entries::HeadsEntries;
use crate::services::{
    AmmlHost, BuildGate, ChatFontCheck, ChatRuntime, ChatState, GeometryReader, HeadsRuntime, HookInstaller,
    PacketObserver, RenderRuntime, RenderState, SkinDiagnostics, TextureBridge,
};
use crate::structs::{
    AnimatedImageOffsets, ClientAccess, PacketOffsets, PlayerListOffsets, ResolvedTextureFns, SkinOffsets,
    SkinPacketOffsets, TextureLayout, TurnedOff, WaypointOffsets,
};
use crate::traits::GameHost;
use crate::types::HookSpec;

// The AMML entry point: load makes the runtime, enable checks the build and installs the hooks, disable removes
// them and leaves the mod manager. The manager link is made on the first ClientInstance::update after enable.
pub struct HeadsMod;

impl HeadsMod {
    // faces waiting between the packet handlers and the render thread
    const CHANNEL_CAPACITY: usize = 256;
    const STORE_CAPACITY: usize = 128;
    const UPLOADS_PER_FRAME: usize = 4;
    // the id as the module string the loader and the hook host take; a test checks it against `AmmlMod::id`
    pub const MODULE: &'static str = "@alaydriem/heads";
    const DIAGNOSTICS_FEATURE: &'static str = "skin diagnostics";
    const GEOMETRY_FEATURE: &'static str = "geometry head locator";
    const PERSONA_FEATURE: &'static str = "persona animated faces";

    // every hook, in install order: the icon blit goes in before the render that tags colours for it, and the
    // render goes in last. They come out in the reverse order
    pub fn hooks() -> [HookSpec; 6] {
        [
            HookSpec {
                entry: HeadsEntries::HUD_LOCATOR_BAR_RENDERER_DRAW_ICON.as_str(),
                table: None,
                detour: draw_icon::detour as *const c_void,
                original: draw_icon::GUARD.original(),
                feature: "the player faces",
            },
            HookSpec {
                entry: HeadsEntries::HANDLE_PLAYER_LIST_PACKET.as_str(),
                table: Some(HeadsEntries::LEGACY_CLIENT_NETWORK_HANDLER_VTABLE.as_str()),
                detour: player_list::detour as *const c_void,
                original: player_list::GUARD.original(),
                feature: "the player faces",
            },
            HookSpec {
                entry: HeadsEntries::HANDLE_PLAYER_SKIN_PACKET.as_str(),
                table: Some(HeadsEntries::LEGACY_CLIENT_NETWORK_HANDLER_VTABLE.as_str()),
                detour: player_skin::detour as *const c_void,
                original: player_skin::GUARD.original(),
                feature: "the player faces",
            },
            HookSpec {
                entry: HeadsEntries::CLIENT_EXIT_LEVEL_HOOK.as_str(),
                table: None,
                detour: exit_level::detour as *const c_void,
                original: exit_level::GUARD.original(),
                feature: "the player faces",
            },
            HookSpec {
                entry: EntryId::CLIENT_INSTANCE_UPDATE.as_str(),
                table: None,
                detour: client_update::detour as *const c_void,
                original: client_update::GUARD.original(),
                feature: "the settings",
            },
            HookSpec {
                entry: HeadsEntries::HUD_LOCATOR_BAR_RENDERER_RENDER.as_str(),
                table: Some(HeadsEntries::HUD_LOCATOR_BAR_RENDERER_VTABLE.as_str()),
                detour: render::detour as *const c_void,
                original: render::GUARD.original(),
                feature: "the player faces",
            },
        ]
    }

    // the chat heads hooks, installed after `hooks()`; when one cannot go in, chat heads is off and the locator bar
    // hooks stay
    pub fn chat_hooks() -> [HookSpec; 2] {
        [
            HookSpec {
                entry: HeadsEntries::GUI_MESSAGE_CTOR.as_str(),
                table: None,
                detour: gui_message::detour as *const c_void,
                original: gui_message::GUARD.original(),
                feature: "chat heads",
            },
            HookSpec {
                entry: HeadsEntries::BITMAP_FONT_GET_CHAR_WIDTH.as_str(),
                table: None,
                detour: char_width::detour as *const c_void,
                original: char_width::GUARD.original(),
                feature: "chat heads",
            },
        ]
    }

    // The body of enable. The build gate comes first: on another build nothing is hooked and one line says why.
    // Then every locator bar entry and hook target must resolve before the state is made or any hook goes in. The
    // chat heads entries and hooks are optional: when one fails, chat heads is off and one line says why.
    pub fn enable_with<H: GameHost>(host: &H, runtime: &HeadsRuntime) -> Result<(), SdkError> {
        runtime.clear_turned_off();
        let mesh_size = host.resolve(HeadsEntries::MESH_SIZE.as_str())?;
        if !BuildGate::accepts(mesh_size) {
            host.warn(&format!(
                "this game build is not supported; the player faces are off (mesh_size={mesh_size:#x})"
            ));
            return Ok(());
        }
        let base_offsets = Self::packet_offsets(host)?;
        let access = Self::client_access(host)?;
        let textures = Self::texture_fns(host)?;
        let texture_layout = Self::texture_layout(host)?;
        let heap = Self::heap(host)?;
        let to_styled_string = Self::value(host, HeadsEntries::JSON_VALUE_TO_STYLED_STRING)?;
        let specs = Self::hooks();
        let targets = HookInstaller::resolve(host, &specs)?;
        let (base_offsets, side_off) = Self::side_features(host, runtime, base_offsets);
        let chat_specs = Self::chat_hooks();
        let chat = Self::chat_parts(host, base_offsets, &chat_specs);
        let offsets = chat.as_ref().map_or(base_offsets, |(offsets, _, _)| *offsets);

        let (sender, receiver) = bounded(Self::CHANNEL_CAPACITY);
        let store = FaceStore::new(Self::STORE_CAPACITY, Self::UPLOADS_PER_FRAME);
        let state = RenderState::new(
            store,
            receiver,
            HeadsSettings::default(),
            TextureBridge::new(textures, texture_layout),
            access,
            Some(heap),
        );
        RenderRuntime::install(state);
        runtime.set_observer(Self::observer(sender.clone(), offsets, to_styled_string, heap));
        if let Err(error) = HookInstaller::install(host, runtime, &specs, &targets) {
            runtime.clear_observer();
            drop(RenderRuntime::uninstall());
            return Err(error);
        }
        let chat = chat.and_then(|(_, font_check, chat_targets)| {
            HookInstaller::install(host, runtime, &chat_specs, &chat_targets)?;
            Ok(font_check)
        });
        match chat {
            Ok(font_check) => {
                ChatRuntime::set_game_access(font_check, heap);
                let flags = ChatRuntime::flags();
                ChatRuntime::install(
                    ChatState::new(GlyphPage::new(ChatState::VANILLA_ROW0), Arc::clone(&flags)),
                    flags,
                );
                if side_off.is_empty() {
                    host.info("features off: none");
                } else {
                    host.info(&format!("features off: {}", side_off.join(", ")));
                }
            }
            Err(error) => {
                runtime.set_observer(Self::observer(sender, base_offsets, to_styled_string, heap));
                host.error(&format!("features off: chat heads ({error})"));
                runtime.note_turned_off(TurnedOff::new("chat heads", error.to_string()));
            }
        }
        runtime.enable_link();
        Ok(())
    }

    // The body of disable: the hooks come out first, so no detour reaches the state, then the state goes. The chat
    // state is only suspended, because its glyph cells must not move for the life of the process.
    pub fn disable_with<H: GameHost>(host: &H, runtime: &HeadsRuntime) {
        HookInstaller::remove_all(host, runtime);
        ChatRuntime::suspend();
        runtime.clear_observer();
        drop(RenderRuntime::uninstall());
        runtime.detach();
    }

    fn observer(
        sender: Sender<FaceUpdate>,
        offsets: PacketOffsets,
        to_styled_string: usize,
        heap: MsvcHeap,
    ) -> PacketObserver {
        PacketObserver::new_with_geometry(
            sender,
            offsets,
            SkinDiagnostics::new(),
            GeometryReader::new(to_styled_string, heap),
        )
    }

    // what chat heads needs before its hooks go in: the packet offsets with the name and XUID, the font check and
    // the hook targets. Any entry that does not resolve turns chat heads off
    fn chat_parts<H: GameHost>(
        host: &H,
        offsets: PacketOffsets,
        specs: &[HookSpec],
    ) -> Result<(PacketOffsets, ChatFontCheck, Vec<u64>), SdkError> {
        let offsets = PacketOffsets {
            add_entry_name: Some(Self::value(host, HeadsEntries::PLAYER_LIST_ADD_ENTRY_NAME)?),
            add_entry_xuid: Some(Self::value(host, HeadsEntries::PLAYER_LIST_ADD_ENTRY_XUID)?),
            ..offsets
        };
        let font_check = ChatFontCheck::new(
            Self::value(host, HeadsEntries::CLIENT_INSTANCE_GET_FONT_HANDLE)?,
            Self::value(host, HeadsEntries::FONT_HANDLE_DTOR)?,
            Self::value(host, HeadsEntries::BITMAP_FONT_VTABLE)?,
            Self::value(host, HeadsEntries::FONT_HANDLE_SIZE)?,
            Self::value(host, HeadsEntries::FONT_HANDLE_DEFAULT_FONT)?,
        )
        .ok_or_else(|| SdkError::Mod("the FontHandle layout does not fit the font check".to_string()))?;
        let targets = HookInstaller::resolve(host, specs)?;
        Ok((offsets, font_check, targets))
    }

    fn value<H: GameHost>(host: &H, id: EntryId) -> Result<usize, SdkError> {
        let value = host.resolve(id.as_str())?;
        usize::try_from(value).map_err(|_| SdkError::Mod(format!("{} does not fit a pointer", id.as_str())))
    }

    fn packet_offsets<H: GameHost>(host: &H) -> Result<PacketOffsets, SdkError> {
        let list = PlayerListOffsets::new(
            Self::value(host, HeadsEntries::PLAYER_LIST_PACKET_ENTRIES)?,
            Self::value(host, HeadsEntries::PLAYER_LIST_ADD_ENTRY_SIZE)?,
            Self::value(host, HeadsEntries::PLAYER_LIST_PACKET_VARIANT_INDEX)?,
            Self::value(host, HeadsEntries::PLAYER_LIST_ADD_ENTRY_UUID)?,
            Self::value(host, HeadsEntries::PLAYER_LIST_ADD_ENTRY_ID)?,
            Self::value(host, HeadsEntries::PLAYER_LIST_ADD_ENTRY_SKIN)?,
        );
        let skin_packet = SkinPacketOffsets::new(
            Self::value(host, HeadsEntries::PLAYER_SKIN_PACKET_UUID)?,
            Self::value(host, HeadsEntries::PLAYER_SKIN_PACKET_SKIN)?,
        );
        let skin = SkinOffsets::new_required(
            Self::value(host, HeadsEntries::SERIALIZED_SKIN_IMPL_SKIN_IMAGE)?,
            Self::value(host, HeadsEntries::SERIALIZED_SKIN_IMPL_GEOMETRY_DATA)?,
        );
        Ok(PacketOffsets::new(list, skin_packet, skin))
    }

    // Adds the side features to the packet offsets. Each feature has its own entries; when one is missing or
    // unusable the feature is off, the entry is logged at error level and the faces stay. Returns the names of the
    // features that are off.
    fn side_features<H: GameHost>(
        host: &H,
        runtime: &HeadsRuntime,
        offsets: PacketOffsets,
    ) -> (PacketOffsets, Vec<&'static str>) {
        let mut off = Vec::new();
        let mut skin = offsets.skin;
        match Self::group(host, runtime, Self::DIAGNOSTICS_FEATURE, HeadsEntries::DIAGNOSTICS) {
            Some([resource_patch, is_premium]) => {
                skin.resource_patch = Some(resource_patch);
                skin.is_premium = Some(is_premium);
            }
            None => off.push(Self::DIAGNOSTICS_FEATURE),
        }
        match Self::group(host, runtime, Self::GEOMETRY_FEATURE, HeadsEntries::GEOMETRY) {
            Some([default_geometry_name]) => skin.default_geometry_name = Some(default_geometry_name),
            None => off.push(Self::GEOMETRY_FEATURE),
        }
        match Self::group(host, runtime, Self::PERSONA_FEATURE, HeadsEntries::PERSONA) {
            Some([list, stride, kind, image, is_persona]) => {
                let animated = AnimatedImageOffsets::new(list, stride, kind, image);
                if animated.is_usable() {
                    skin.animated = Some(animated);
                    skin.is_persona = Some(is_persona);
                } else {
                    Self::feature_off(host, runtime, Self::PERSONA_FEATURE, HeadsEntries::ANIMATED_IMAGE_DATA_SIZE);
                    off.push(Self::PERSONA_FEATURE);
                }
            }
            None => off.push(Self::PERSONA_FEATURE),
        }
        (PacketOffsets { skin, ..offsets }, off)
    }

    // The values of an optional group of entries, or `None` when any of them is missing or unusable. Every entry
    // that fails is logged.
    fn group<H: GameHost, const N: usize>(
        host: &H,
        runtime: &HeadsRuntime,
        feature: &'static str,
        ids: [EntryId; N],
    ) -> Option<[usize; N]> {
        let mut values = [0usize; N];
        let mut usable = true;
        for (slot, id) in values.iter_mut().zip(ids) {
            match Self::value(host, id) {
                Ok(value) => *slot = value,
                Err(_) => {
                    usable = false;
                    Self::feature_off(host, runtime, feature, id);
                }
            }
        }
        usable.then_some(values)
    }

    // The loader log shows the line at error level now. The manager sink does not exist yet at enable, so the
    // runtime holds the feature and writes it to the manager log when the link attaches.
    fn feature_off<H: GameHost>(host: &H, runtime: &HeadsRuntime, feature: &'static str, id: EntryId) {
        host.error(&format!("game entry missing or unusable, {feature} turned off ({})", id.as_str()));
        runtime.note_turned_off(TurnedOff::new(feature, id.as_str()));
    }

    fn client_access<H: GameHost>(host: &H) -> Result<ClientAccess, SdkError> {
        let waypoints = WaypointOffsets::new(
            Self::value(host, HeadsEntries::CLIENT_LOCATOR_BAR_LIST_HEAD)?,
            Self::value(host, HeadsEntries::CLIENT_LOCATOR_BAR_NODE_WAYPOINT)?,
            Self::value(host, HeadsEntries::CLIENT_WAYPOINT_COLOR)?,
            Self::value(host, HeadsEntries::CLIENT_WAYPOINT_COLOR_ENGAGED)?,
            Self::value(host, HeadsEntries::CLIENT_WAYPOINT_VISIBLE)?,
            Self::value(host, HeadsEntries::CLIENT_ENTITY_WAYPOINT_ACTOR_ID)?,
        )
        .ok_or_else(|| {
            SdkError::Mod("the ClientWaypoint colour layout does not fit the waypoint tagger".to_string())
        })?;
        Ok(ClientAccess::new(
            Self::value(host, EntryId::CLIENT_INSTANCE_GET_LOCAL_PLAYER)?,
            Self::value(host, HeadsEntries::CLIENT_INSTANCE_GET_TEXTURE_GROUP)?,
            Self::value(host, HeadsEntries::LOCAL_PLAYER_CLIENT_LOCATOR_BAR)?,
            Self::value(host, HeadsEntries::CLIENT_ENTITY_WAYPOINT_VTABLE)?,
            waypoints,
        ))
    }

    fn texture_layout<H: GameHost>(host: &H) -> Result<TextureLayout, SdkError> {
        TextureLayout::new(
            Self::value(host, HeadsEntries::TEXTURE_GROUP_TEXTURE_GROUP_BASE)?,
            Self::value(host, HeadsEntries::IMAGE_BUFFER_SIZE)?,
            Self::value(host, HeadsEntries::IMAGE_BUFFER_DELETER)?,
            Self::value(host, HeadsEntries::IMAGE_BUFFER_DATA)?,
            Self::value(host, HeadsEntries::IMAGE_BUFFER_BYTE_SIZE)?,
            Self::value(host, HeadsEntries::RESOURCE_LOCATION_SIZE)?,
        )
        .ok_or_else(|| {
            SdkError::Mod("the ImageBuffer or ResourceLocation layout does not fit the texture bridge".to_string())
        })
    }

    fn texture_fns<H: GameHost>(host: &H) -> Result<ResolvedTextureFns, SdkError> {
        Ok(ResolvedTextureFns {
            resource_location_ctor: Self::value(host, HeadsEntries::RESOURCE_LOCATION_CTOR)?,
            image_buffer_ctor: Self::value(host, HeadsEntries::IMAGE_BUFFER_CTOR_FROM_DESCRIPTION)?,
            upload: Self::value(host, HeadsEntries::TEXTURE_GROUP_UPLOAD_TEXTURE)?,
            get: Self::value(host, HeadsEntries::TEXTURE_GROUP_GET_TEXTURE)?,
            unload: Self::value(host, HeadsEntries::TEXTURE_GROUP_UNLOAD_TEXTURE)?,
        })
    }

    fn heap<H: GameHost>(host: &H) -> Result<MsvcHeap, SdkError> {
        let new = Self::value(host, EntryId::CORE_OPERATOR_NEW)?;
        let delete = Self::value(host, EntryId::CORE_OPERATOR_DELETE)?;
        // SAFETY: the two entries resolve to the game operator new(size_t) and sized operator delete(void*, size_t),
        // which have these signatures.
        let (new, delete) = unsafe {
            (
                std::mem::transmute::<usize, unsafe extern "C" fn(usize) -> *mut u8>(new),
                std::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, usize)>(delete),
            )
        };
        Ok(MsvcHeap::new(new, delete))
    }
}

impl AmmlMod for HeadsMod {
    const NAMESPACE: &'static str = "alaydriem";
    const NAME: &'static str = "heads";

    fn load(api: &Api) -> Result<(), SdkError> {
        let dirs = ModDirs::new(api, Self::MODULE)?;
        HeadsRuntime::install(&dirs.config_dir);
        Ok(())
    }

    fn enable(api: &Api) -> Result<(), SdkError> {
        let runtime = HeadsRuntime::get().ok_or_else(|| SdkError::Mod("load did not run".to_string()))?;
        Self::enable_with(&AmmlHost::new(api, Self::MODULE), runtime)
    }

    fn disable(api: &Api) {
        let Some(runtime) = HeadsRuntime::get() else { return };
        Self::disable_with(&AmmlHost::new(api, Self::MODULE), runtime);
    }
}
