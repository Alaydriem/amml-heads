use alaydriem_heads::structs::{
    AnimatedImageOffsets, PacketOffsets, PlayerListOffsets, SkinOffsets, SkinPacketOffsets, TextureLayout,
    WaypointOffsets,
};

use super::fake_packet::{ADD_ENTRY_NAME, ADD_ENTRY_SKIN, ADD_ENTRY_XUID, ENTRY_SIZE, VARIANT_INDEX};

/// The 26.60 values of the layout entries, as the fakes lay out their memory.
pub struct FakeLayout;

impl FakeLayout {
    pub const ENTRIES: usize = 0x30;
    pub const ENTRY_UUID: usize = 0x08;
    pub const ENTRY_ACTOR: usize = 0x18;
    pub const SKIN_PACKET_UUID: usize = 0x30;
    pub const SKIN_PACKET_SKIN: usize = 0x40;
    pub const SKIN_IMAGE: usize = 0x80;
    pub const GEOMETRY_DATA: usize = 0xF8;
    pub const RESOURCE_PATCH: usize = 0x40;
    pub const DEFAULT_GEOMETRY_NAME: usize = 0x60;
    pub const IS_PREMIUM: usize = 0x1E9;
    pub const IS_PERSONA: usize = 0x1EA;
    pub const ANIMATED_IMAGES: usize = 0xE0;
    pub const ANIMATED_IMAGE_SIZE: usize = 0x40;
    pub const ANIMATED_IMAGE_TYPE: usize = 0;
    pub const ANIMATED_IMAGE_IMAGE: usize = 8;
    pub const LIST_HEAD: usize = 0x10;
    pub const NODE_WAYPOINT: usize = 0x20;
    pub const COLOR: usize = 0x18;
    pub const COLOR_ENGAGED: usize = 0x28;
    pub const VISIBLE: usize = 0x60;
    pub const ACTOR_ID: usize = 0x78;
    pub const TEXTURE_GROUP_BASE: usize = 0x18;
    pub const IMAGE_BUFFER_SIZE: usize = 0x30;
    pub const IMAGE_BUFFER_DELETER: usize = 0;
    pub const IMAGE_BUFFER_DATA: usize = 8;
    pub const IMAGE_BUFFER_BYTE_SIZE: usize = 0x10;
    pub const RESOURCE_LOCATION_SIZE: usize = 0x38;
    pub const FONT_HANDLE_SIZE: usize = 0x50;
    pub const FONT_HANDLE_DEFAULT_FONT: usize = 0x30;

    pub fn packet_offsets() -> PacketOffsets {
        PacketOffsets::new(Self::list(), Self::skin_packet(), Self::skin())
    }

    pub fn packet_offsets_with_chat() -> PacketOffsets {
        PacketOffsets::new_with_chat(Self::list(), Self::skin_packet(), Self::skin(), ADD_ENTRY_NAME, ADD_ENTRY_XUID)
    }

    pub fn waypoints() -> WaypointOffsets {
        WaypointOffsets::new(
            Self::LIST_HEAD,
            Self::NODE_WAYPOINT,
            Self::COLOR,
            Self::COLOR_ENGAGED,
            Self::VISIBLE,
            Self::ACTOR_ID,
        )
        .unwrap()
    }

    pub fn texture_layout() -> TextureLayout {
        TextureLayout::new(
            Self::TEXTURE_GROUP_BASE,
            Self::IMAGE_BUFFER_SIZE,
            Self::IMAGE_BUFFER_DELETER,
            Self::IMAGE_BUFFER_DATA,
            Self::IMAGE_BUFFER_BYTE_SIZE,
            Self::RESOURCE_LOCATION_SIZE,
        )
        .unwrap()
    }

    fn list() -> PlayerListOffsets {
        PlayerListOffsets::new(
            Self::ENTRIES,
            ENTRY_SIZE,
            VARIANT_INDEX,
            Self::ENTRY_UUID,
            Self::ENTRY_ACTOR,
            ADD_ENTRY_SKIN,
        )
    }

    fn skin_packet() -> SkinPacketOffsets {
        SkinPacketOffsets::new(Self::SKIN_PACKET_UUID, Self::SKIN_PACKET_SKIN)
    }

    fn skin() -> SkinOffsets {
        SkinOffsets::new(
            Self::SKIN_IMAGE,
            Self::GEOMETRY_DATA,
            Some(Self::RESOURCE_PATCH),
            Some(Self::DEFAULT_GEOMETRY_NAME),
            Some(Self::IS_PREMIUM),
            Some(Self::IS_PERSONA),
            Some(AnimatedImageOffsets::new(
                Self::ANIMATED_IMAGES,
                Self::ANIMATED_IMAGE_SIZE,
                Self::ANIMATED_IMAGE_TYPE,
                Self::ANIMATED_IMAGE_IMAGE,
            )),
        )
    }
}
