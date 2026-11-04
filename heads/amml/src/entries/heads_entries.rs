use amml_game::EntryId;

// the entries only heads reads; the ones every mod shares are amml_game::EntryId constants
pub struct HeadsEntries;

impl HeadsEntries {
    pub const HUD_LOCATOR_BAR_RENDERER_VTABLE: EntryId = EntryId::new("HudLocatorBarRenderer.vtable");
    pub const HUD_LOCATOR_BAR_RENDERER_RENDER: EntryId = EntryId::new("HudLocatorBarRenderer.render");
    pub const HUD_LOCATOR_BAR_RENDERER_DRAW_ICON: EntryId = EntryId::new("HudLocatorBarRenderer.drawIcon");
    pub const LOCAL_PLAYER_CLIENT_LOCATOR_BAR: EntryId = EntryId::new("LocalPlayer.mClientLocatorBar");
    pub const CLIENT_INSTANCE_GET_TEXTURE_GROUP: EntryId = EntryId::new("ClientInstance.getTextureGroup");
    pub const CLIENT_ENTITY_WAYPOINT_VTABLE: EntryId = EntryId::new("ClientEntityWaypoint.vtable");
    pub const HANDLE_PLAYER_LIST_PACKET: EntryId = EntryId::new("ClientNetworkHandler.handle.PlayerListPacket");
    pub const HANDLE_PLAYER_SKIN_PACKET: EntryId = EntryId::new("ClientNetworkHandler.handle.PlayerSkinPacket");
    pub const TEXTURE_GROUP_GET_TEXTURE: EntryId = EntryId::new("TextureGroup.getTexture");
    pub const TEXTURE_GROUP_UPLOAD_TEXTURE: EntryId = EntryId::new("TextureGroup.uploadTexture");
    pub const TEXTURE_GROUP_UNLOAD_TEXTURE: EntryId = EntryId::new("TextureGroup.unloadTexture");
    pub const RESOURCE_LOCATION_CTOR: EntryId = EntryId::new("ResourceLocation.ctor");
    pub const IMAGE_BUFFER_CTOR_FROM_DESCRIPTION: EntryId = EntryId::new("ImageBuffer.ctorFromDescription");
    pub const PLAYER_LIST_ADD_ENTRY_SIZE: EntryId = EntryId::new("PlayerListAddEntry.size");
    pub const PLAYER_LIST_PACKET_VARIANT_INDEX: EntryId = EntryId::new("PlayerListPacket.variantIndex");
    pub const PLAYER_LIST_ADD_ENTRY_SKIN: EntryId = EntryId::new("PlayerListAddEntry.mSkin");
    pub const SERIALIZED_SKIN_IMPL_SKIN_IMAGE: EntryId = EntryId::new("SerializedSkinImpl.mSkinImage");

    pub const LEGACY_CLIENT_NETWORK_HANDLER_VTABLE: EntryId = EntryId::new("LegacyClientNetworkHandler.vtable");
    pub const CLIENT_EXIT_LEVEL_HOOK: EntryId = EntryId::new("ClientExitLevel.hook");
    pub const MESH_SIZE: EntryId = EntryId::new("Mesh.size");
    pub const JSON_VALUE_TO_STYLED_STRING: EntryId = EntryId::new("JsonValue.toStyledString");
    pub const SERIALIZED_SKIN_IMPL_GEOMETRY_DATA: EntryId = EntryId::new("SerializedSkinImpl.mGeometryData");

    pub const GUI_MESSAGE_CTOR: EntryId = EntryId::new("GuiMessage.ctor");
    pub const BITMAP_FONT_VTABLE: EntryId = EntryId::new("BitmapFont.vtable");
    pub const BITMAP_FONT_GET_CHAR_WIDTH: EntryId = EntryId::new("BitmapFont._getCharWidth");
    pub const PLAYER_LIST_ADD_ENTRY_NAME: EntryId = EntryId::new("PlayerListAddEntry.mName");
    pub const PLAYER_LIST_ADD_ENTRY_XUID: EntryId = EntryId::new("PlayerListAddEntry.mXUID");
    pub const CLIENT_INSTANCE_GET_FONT_HANDLE: EntryId = EntryId::new("ClientInstance.getFontHandle");
    pub const FONT_HANDLE_DTOR: EntryId = EntryId::new("FontHandle.dtor");
    pub const FONT_HANDLE_SIZE: EntryId = EntryId::new("FontHandle.size");
    pub const FONT_HANDLE_DEFAULT_FONT: EntryId = EntryId::new("FontHandle.mDefaultFont");

    pub const PLAYER_LIST_PACKET_ENTRIES: EntryId = EntryId::new("PlayerListPacket.mEntries");
    pub const PLAYER_LIST_ADD_ENTRY_UUID: EntryId = EntryId::new("PlayerListAddEntry.mUUID");
    pub const PLAYER_LIST_ADD_ENTRY_ID: EntryId = EntryId::new("PlayerListAddEntry.mId");
    pub const PLAYER_SKIN_PACKET_UUID: EntryId = EntryId::new("PlayerSkinPacket.mUUID");
    pub const PLAYER_SKIN_PACKET_SKIN: EntryId = EntryId::new("PlayerSkinPacket.mSkin");
    pub const SERIALIZED_SKIN_IMPL_RESOURCE_PATCH: EntryId = EntryId::new("SerializedSkinImpl.mResourcePatch");
    pub const SERIALIZED_SKIN_IMPL_DEFAULT_GEOMETRY_NAME: EntryId =
        EntryId::new("SerializedSkinImpl.mDefaultGeometryName");
    pub const SERIALIZED_SKIN_IMPL_SKIN_ANIMATED_IMAGES: EntryId =
        EntryId::new("SerializedSkinImpl.mSkinAnimatedImages");
    pub const SERIALIZED_SKIN_IMPL_IS_PREMIUM: EntryId = EntryId::new("SerializedSkinImpl.mIsPremium");
    pub const SERIALIZED_SKIN_IMPL_IS_PERSONA: EntryId = EntryId::new("SerializedSkinImpl.mIsPersona");
    pub const ANIMATED_IMAGE_DATA_SIZE: EntryId = EntryId::new("AnimatedImageData.size");
    pub const ANIMATED_IMAGE_DATA_TYPE: EntryId = EntryId::new("AnimatedImageData.mType");
    pub const ANIMATED_IMAGE_DATA_IMAGE: EntryId = EntryId::new("AnimatedImageData.mImage");
    pub const CLIENT_LOCATOR_BAR_LIST_HEAD: EntryId = EntryId::new("ClientLocatorBar.listHead");
    pub const CLIENT_LOCATOR_BAR_NODE_WAYPOINT: EntryId = EntryId::new("ClientLocatorBar.nodeWaypoint");
    pub const CLIENT_WAYPOINT_COLOR: EntryId = EntryId::new("ClientWaypoint.mColor");
    pub const CLIENT_WAYPOINT_COLOR_ENGAGED: EntryId = EntryId::new("ClientWaypoint.mColorEngaged");
    pub const CLIENT_WAYPOINT_VISIBLE: EntryId = EntryId::new("ClientWaypoint.mVisible");
    pub const CLIENT_ENTITY_WAYPOINT_ACTOR_ID: EntryId = EntryId::new("ClientEntityWaypoint.mActorId");
    pub const TEXTURE_GROUP_TEXTURE_GROUP_BASE: EntryId = EntryId::new("TextureGroup.textureGroupBase");
    pub const IMAGE_BUFFER_SIZE: EntryId = EntryId::new("ImageBuffer.size");
    pub const IMAGE_BUFFER_DELETER: EntryId = EntryId::new("ImageBuffer.mDeleter");
    pub const IMAGE_BUFFER_DATA: EntryId = EntryId::new("ImageBuffer.mData");
    pub const IMAGE_BUFFER_BYTE_SIZE: EntryId = EntryId::new("ImageBuffer.mSize");
    pub const RESOURCE_LOCATION_SIZE: EntryId = EntryId::new("ResourceLocation.size");

    /// Every entry the mod resolves, which `amml.hcl` lists.
    pub const MANIFEST: [EntryId; 60] = [
        EntryId::CLIENT_INSTANCE_UPDATE,
        EntryId::CLIENT_INSTANCE_GET_LOCAL_PLAYER,
        Self::CLIENT_INSTANCE_GET_TEXTURE_GROUP,
        Self::CLIENT_EXIT_LEVEL_HOOK,
        Self::LEGACY_CLIENT_NETWORK_HANDLER_VTABLE,
        Self::MESH_SIZE,
        EntryId::CORE_OPERATOR_NEW,
        EntryId::CORE_OPERATOR_DELETE,
        Self::HUD_LOCATOR_BAR_RENDERER_VTABLE,
        Self::HUD_LOCATOR_BAR_RENDERER_RENDER,
        Self::HUD_LOCATOR_BAR_RENDERER_DRAW_ICON,
        Self::LOCAL_PLAYER_CLIENT_LOCATOR_BAR,
        Self::CLIENT_ENTITY_WAYPOINT_VTABLE,
        Self::HANDLE_PLAYER_LIST_PACKET,
        Self::HANDLE_PLAYER_SKIN_PACKET,
        Self::TEXTURE_GROUP_GET_TEXTURE,
        Self::TEXTURE_GROUP_UPLOAD_TEXTURE,
        Self::TEXTURE_GROUP_UNLOAD_TEXTURE,
        Self::RESOURCE_LOCATION_CTOR,
        Self::IMAGE_BUFFER_CTOR_FROM_DESCRIPTION,
        Self::PLAYER_LIST_ADD_ENTRY_SIZE,
        Self::PLAYER_LIST_PACKET_VARIANT_INDEX,
        Self::PLAYER_LIST_ADD_ENTRY_SKIN,
        Self::SERIALIZED_SKIN_IMPL_SKIN_IMAGE,
        Self::JSON_VALUE_TO_STYLED_STRING,
        Self::SERIALIZED_SKIN_IMPL_GEOMETRY_DATA,
        Self::GUI_MESSAGE_CTOR,
        Self::BITMAP_FONT_VTABLE,
        Self::BITMAP_FONT_GET_CHAR_WIDTH,
        Self::PLAYER_LIST_ADD_ENTRY_NAME,
        Self::PLAYER_LIST_ADD_ENTRY_XUID,
        Self::CLIENT_INSTANCE_GET_FONT_HANDLE,
        Self::FONT_HANDLE_DTOR,
        Self::FONT_HANDLE_SIZE,
        Self::FONT_HANDLE_DEFAULT_FONT,
        Self::PLAYER_LIST_PACKET_ENTRIES,
        Self::PLAYER_LIST_ADD_ENTRY_UUID,
        Self::PLAYER_LIST_ADD_ENTRY_ID,
        Self::PLAYER_SKIN_PACKET_UUID,
        Self::PLAYER_SKIN_PACKET_SKIN,
        Self::SERIALIZED_SKIN_IMPL_RESOURCE_PATCH,
        Self::SERIALIZED_SKIN_IMPL_DEFAULT_GEOMETRY_NAME,
        Self::SERIALIZED_SKIN_IMPL_SKIN_ANIMATED_IMAGES,
        Self::SERIALIZED_SKIN_IMPL_IS_PREMIUM,
        Self::SERIALIZED_SKIN_IMPL_IS_PERSONA,
        Self::ANIMATED_IMAGE_DATA_SIZE,
        Self::ANIMATED_IMAGE_DATA_TYPE,
        Self::ANIMATED_IMAGE_DATA_IMAGE,
        Self::CLIENT_LOCATOR_BAR_LIST_HEAD,
        Self::CLIENT_LOCATOR_BAR_NODE_WAYPOINT,
        Self::CLIENT_WAYPOINT_COLOR,
        Self::CLIENT_WAYPOINT_COLOR_ENGAGED,
        Self::CLIENT_WAYPOINT_VISIBLE,
        Self::CLIENT_ENTITY_WAYPOINT_ACTOR_ID,
        Self::TEXTURE_GROUP_TEXTURE_GROUP_BASE,
        Self::IMAGE_BUFFER_SIZE,
        Self::IMAGE_BUFFER_DELETER,
        Self::IMAGE_BUFFER_DATA,
        Self::IMAGE_BUFFER_BYTE_SIZE,
        Self::RESOURCE_LOCATION_SIZE,
    ];

    pub const ALL: [EntryId; 17] = [
        Self::HUD_LOCATOR_BAR_RENDERER_VTABLE,
        Self::HUD_LOCATOR_BAR_RENDERER_RENDER,
        Self::HUD_LOCATOR_BAR_RENDERER_DRAW_ICON,
        Self::LOCAL_PLAYER_CLIENT_LOCATOR_BAR,
        Self::CLIENT_INSTANCE_GET_TEXTURE_GROUP,
        Self::CLIENT_ENTITY_WAYPOINT_VTABLE,
        Self::HANDLE_PLAYER_LIST_PACKET,
        Self::HANDLE_PLAYER_SKIN_PACKET,
        Self::TEXTURE_GROUP_GET_TEXTURE,
        Self::TEXTURE_GROUP_UPLOAD_TEXTURE,
        Self::TEXTURE_GROUP_UNLOAD_TEXTURE,
        Self::RESOURCE_LOCATION_CTOR,
        Self::IMAGE_BUFFER_CTOR_FROM_DESCRIPTION,
        Self::PLAYER_LIST_ADD_ENTRY_SIZE,
        Self::PLAYER_LIST_PACKET_VARIANT_INDEX,
        Self::PLAYER_LIST_ADD_ENTRY_SKIN,
        Self::SERIALIZED_SKIN_IMPL_SKIN_IMAGE,
    ];

    /// The entries chat heads needs; a missing one turns off chat heads only.
    pub const CHAT: [EntryId; 9] = [
        Self::GUI_MESSAGE_CTOR,
        Self::BITMAP_FONT_VTABLE,
        Self::BITMAP_FONT_GET_CHAR_WIDTH,
        Self::PLAYER_LIST_ADD_ENTRY_NAME,
        Self::PLAYER_LIST_ADD_ENTRY_XUID,
        Self::CLIENT_INSTANCE_GET_FONT_HANDLE,
        Self::FONT_HANDLE_DTOR,
        Self::FONT_HANDLE_SIZE,
        Self::FONT_HANDLE_DEFAULT_FONT,
    ];

    /// The game layout values the locator bar needs; a missing one turns off the mod.
    pub const LAYOUT: [EntryId; 17] = [
        Self::PLAYER_LIST_PACKET_ENTRIES,
        Self::PLAYER_LIST_ADD_ENTRY_UUID,
        Self::PLAYER_LIST_ADD_ENTRY_ID,
        Self::PLAYER_SKIN_PACKET_UUID,
        Self::PLAYER_SKIN_PACKET_SKIN,
        Self::CLIENT_LOCATOR_BAR_LIST_HEAD,
        Self::CLIENT_LOCATOR_BAR_NODE_WAYPOINT,
        Self::CLIENT_WAYPOINT_COLOR,
        Self::CLIENT_WAYPOINT_COLOR_ENGAGED,
        Self::CLIENT_WAYPOINT_VISIBLE,
        Self::CLIENT_ENTITY_WAYPOINT_ACTOR_ID,
        Self::TEXTURE_GROUP_TEXTURE_GROUP_BASE,
        Self::IMAGE_BUFFER_SIZE,
        Self::IMAGE_BUFFER_DELETER,
        Self::IMAGE_BUFFER_DATA,
        Self::IMAGE_BUFFER_BYTE_SIZE,
        Self::RESOURCE_LOCATION_SIZE,
    ];

    /// The entries only the skin diagnostics log reads; a missing one turns off the diagnostics only.
    pub const DIAGNOSTICS: [EntryId; 2] =
        [Self::SERIALIZED_SKIN_IMPL_RESOURCE_PATCH, Self::SERIALIZED_SKIN_IMPL_IS_PREMIUM];

    /// The entry the geometry head locator reads; a missing one turns off the locator only.
    pub const GEOMETRY: [EntryId; 1] = [Self::SERIALIZED_SKIN_IMPL_DEFAULT_GEOMETRY_NAME];

    /// The entries the persona animated face reads, in the order `AnimatedImageOffsets::new` takes them with the
    /// persona flag last; a missing one turns off the persona animated face only.
    pub const PERSONA: [EntryId; 5] = [
        Self::SERIALIZED_SKIN_IMPL_SKIN_ANIMATED_IMAGES,
        Self::ANIMATED_IMAGE_DATA_SIZE,
        Self::ANIMATED_IMAGE_DATA_TYPE,
        Self::ANIMATED_IMAGE_DATA_IMAGE,
        Self::SERIALIZED_SKIN_IMPL_IS_PERSONA,
    ];
}
