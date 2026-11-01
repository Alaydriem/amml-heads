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

    /// Every entry the mod resolves, which `amml.hcl` lists.
    pub const MANIFEST: [EntryId; 33] = [
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
    pub const CHAT: [EntryId; 7] = [
        Self::GUI_MESSAGE_CTOR,
        Self::BITMAP_FONT_VTABLE,
        Self::BITMAP_FONT_GET_CHAR_WIDTH,
        Self::PLAYER_LIST_ADD_ENTRY_NAME,
        Self::PLAYER_LIST_ADD_ENTRY_XUID,
        Self::CLIENT_INSTANCE_GET_FONT_HANDLE,
        Self::FONT_HANDLE_DTOR,
    ];
}
