use std::cell::RefCell;
use std::collections::BTreeMap;

use alaydriem_heads::entries::HeadsEntries;
use alaydriem_heads::traits::GameHost;
use alaydriem_heads::types::HookSpec;
use amml_game::EntryId;
use amml_sdk::{HookHandle, SdkError};

use super::{ENTITY_VTABLE, FakeGame, FakeGeometry};

const HUD_VTABLE: u64 = 0xF260170;
const LEGACY_VTABLE: u64 = 0xF2CFE40;

/// A loader stand-in with the 26.60 entry values; hooks are recorded, never installed.
pub struct FakeHost {
    entries: BTreeMap<&'static str, u64>,
    pointers: BTreeMap<u64, u64>,
    hooked: RefCell<Vec<&'static str>>,
    unhooked: RefCell<Vec<HookHandle>>,
    warnings: RefCell<Vec<String>>,
    infos: RefCell<Vec<String>>,
    failing: Option<&'static str>,
}

impl FakeHost {
    pub const DRAW_ICON: u64 = 0x598A9D0;
    pub const RENDER: u64 = 0x5745690;
    pub const PLAYER_LIST: u64 = 0x6BDD410;
    pub const PLAYER_SKIN: u64 = 0x6BDEC10;
    pub const GUI_MESSAGE: u64 = 0xE8D420;
    pub const CHAR_WIDTH: u64 = 0x15902C0;

    pub fn for_2660() -> Self {
        let textures = FakeGame::texture_fns();
        let (heap_new, heap_delete) = FakeGame::heap_addresses();
        let entries: [(EntryId, u64); 25] = [
            (HeadsEntries::MESH_SIZE, 0x260),
            (HeadsEntries::HUD_LOCATOR_BAR_RENDERER_VTABLE, HUD_VTABLE),
            (HeadsEntries::HUD_LOCATOR_BAR_RENDERER_RENDER, 16),
            (HeadsEntries::HUD_LOCATOR_BAR_RENDERER_DRAW_ICON, Self::DRAW_ICON),
            (HeadsEntries::LOCAL_PLAYER_CLIENT_LOCATOR_BAR, 0xEF0),
            (HeadsEntries::CLIENT_INSTANCE_GET_TEXTURE_GROUP, 103),
            (EntryId::CLIENT_INSTANCE_GET_LOCAL_PLAYER, 30),
            (EntryId::CLIENT_INSTANCE_UPDATE, 0x1000),
            (HeadsEntries::CLIENT_EXIT_LEVEL_HOOK, 0x3000),
            (HeadsEntries::LEGACY_CLIENT_NETWORK_HANDLER_VTABLE, LEGACY_VTABLE),
            (HeadsEntries::CLIENT_ENTITY_WAYPOINT_VTABLE, ENTITY_VTABLE as u64),
            (HeadsEntries::HANDLE_PLAYER_LIST_PACKET, 179),
            (HeadsEntries::HANDLE_PLAYER_SKIN_PACKET, 107),
            (HeadsEntries::TEXTURE_GROUP_GET_TEXTURE, textures.get as u64),
            (HeadsEntries::TEXTURE_GROUP_UPLOAD_TEXTURE, textures.upload as u64),
            (HeadsEntries::TEXTURE_GROUP_UNLOAD_TEXTURE, textures.unload as u64),
            (HeadsEntries::RESOURCE_LOCATION_CTOR, textures.resource_location_ctor as u64),
            (HeadsEntries::IMAGE_BUFFER_CTOR_FROM_DESCRIPTION, textures.image_buffer_ctor as u64),
            (HeadsEntries::PLAYER_LIST_ADD_ENTRY_SIZE, 0xD8),
            (HeadsEntries::PLAYER_LIST_PACKET_VARIANT_INDEX, 0xD0),
            (HeadsEntries::PLAYER_LIST_ADD_ENTRY_SKIN, 0xA8),
            (HeadsEntries::SERIALIZED_SKIN_IMPL_SKIN_IMAGE, 0x80),
            (EntryId::CORE_OPERATOR_NEW, heap_new as u64),
            (HeadsEntries::JSON_VALUE_TO_STYLED_STRING, FakeGeometry::to_styled_string_address() as u64),
            (HeadsEntries::SERIALIZED_SKIN_IMPL_GEOMETRY_DATA, 0xF8),
        ];
        let mut entries: BTreeMap<&'static str, u64> =
            entries.iter().map(|(id, value)| (id.as_str(), *value)).collect();
        entries.insert(EntryId::CORE_OPERATOR_DELETE.as_str(), heap_delete as u64);
        let chat: [(EntryId, u64); 7] = [
            (HeadsEntries::GUI_MESSAGE_CTOR, Self::GUI_MESSAGE),
            (HeadsEntries::BITMAP_FONT_VTABLE, 0xF0EA8A0),
            (HeadsEntries::BITMAP_FONT_GET_CHAR_WIDTH, Self::CHAR_WIDTH),
            (HeadsEntries::PLAYER_LIST_ADD_ENTRY_NAME, 0x20),
            (HeadsEntries::PLAYER_LIST_ADD_ENTRY_XUID, 0x40),
            (HeadsEntries::CLIENT_INSTANCE_GET_FONT_HANDLE, 87),
            (HeadsEntries::FONT_HANDLE_DTOR, 0xE7F9E0),
        ];
        entries.extend(chat.iter().map(|(id, value)| (id.as_str(), *value)));
        let pointers = BTreeMap::from([
            (HUD_VTABLE + 16 * 8, Self::RENDER),
            (LEGACY_VTABLE + 179 * 8, Self::PLAYER_LIST),
            (LEGACY_VTABLE + 107 * 8, Self::PLAYER_SKIN),
        ]);
        Self {
            entries,
            pointers,
            hooked: RefCell::new(Vec::new()),
            unhooked: RefCell::new(Vec::new()),
            warnings: RefCell::new(Vec::new()),
            infos: RefCell::new(Vec::new()),
            failing: None,
        }
    }

    pub fn with_entry(mut self, id: EntryId, value: u64) -> Self {
        self.entries.insert(id.as_str(), value);
        self
    }

    pub fn without_entry(mut self, id: EntryId) -> Self {
        self.entries.remove(id.as_str());
        self
    }

    /// The hook of `entry` fails to install.
    pub fn with_failing_hook(mut self, entry: EntryId) -> Self {
        self.failing = Some(entry.as_str());
        self
    }

    pub fn hooked(&self) -> Vec<&'static str> {
        self.hooked.borrow().clone()
    }

    pub fn unhooked(&self) -> Vec<HookHandle> {
        self.unhooked.borrow().clone()
    }

    pub fn infos(&self) -> Vec<String> {
        self.infos.borrow().clone()
    }

    pub fn warnings(&self) -> Vec<String> {
        self.warnings.borrow().clone()
    }
}

impl GameHost for FakeHost {
    fn resolve(&self, id: &str) -> Result<u64, SdkError> {
        self.entries.get(id).copied().ok_or_else(|| SdkError::MissingEntry(id.to_string()))
    }

    fn read_pointer(&self, address: u64) -> Result<u64, SdkError> {
        self.pointers.get(&address).copied().ok_or_else(|| SdkError::Mod(format!("nothing at {address:#x}")))
    }

    fn hook(&self, _target: u64, spec: &HookSpec) -> Result<HookHandle, SdkError> {
        if self.failing == Some(spec.entry) {
            return Err(SdkError::Mod(format!("{} would not hook", spec.entry)));
        }
        let mut hooked = self.hooked.borrow_mut();
        hooked.push(spec.entry);
        Ok(HookHandle(hooked.len() as u64))
    }

    fn unhook(&self, hook: HookHandle) -> Result<(), SdkError> {
        self.unhooked.borrow_mut().push(hook);
        Ok(())
    }

    fn warn(&self, message: &str) {
        self.warnings.borrow_mut().push(message.to_string());
    }

    fn info(&self, message: &str) {
        self.infos.borrow_mut().push(message.to_string());
    }
}
