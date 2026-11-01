/// The offsets that moved between builds, read from the field entries when the mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketOffsets {
    /// `sizeof(PlayerListPacket::Entry)`, the stride of the entries vector.
    pub entry_size: usize,
    /// The byte in an entry that selects Remove (0) or Add (1).
    pub variant_index: usize,
    /// `mSkin` (a `shared_ptr`) inside an Add entry.
    pub add_entry_skin: usize,
    /// `mSkinImage` (an `mce::Image`) inside `SerializedSkinImpl`.
    pub skin_image: usize,
    /// `mGeometryData` (a `Json::Value`) inside `SerializedSkinImpl`.
    pub geometry_data: usize,
    /// `mName` inside an Add entry; `None` when chat heads is off.
    pub add_entry_name: Option<usize>,
    /// `mXUID` inside an Add entry; `None` when chat heads is off.
    pub add_entry_xuid: Option<usize>,
}

impl PacketOffsets {
    /// The offsets that hold in every supported build.
    pub const ENTRIES_VECTOR: usize = 0x30;
    pub const ENTRY_UUID: usize = 0x08;
    pub const ENTRY_ACTOR: usize = 0x18;
    pub const SKIN_PACKET_UUID: usize = 0x30;
    pub const SKIN_PACKET_SKIN: usize = 0x40;
    pub const VARIANT_REMOVE: u8 = 0;
    pub const VARIANT_ADD: u8 = 1;

    pub fn new(
        entry_size: usize,
        variant_index: usize,
        add_entry_skin: usize,
        skin_image: usize,
        geometry_data: usize,
    ) -> Self {
        Self {
            entry_size,
            variant_index,
            add_entry_skin,
            skin_image,
            geometry_data,
            add_entry_name: None,
            add_entry_xuid: None,
        }
    }

    /// As `new`, with the Add entry name and XUID offsets that the chat feed reads.
    pub fn new_with_chat(
        entry_size: usize,
        variant_index: usize,
        add_entry_skin: usize,
        skin_image: usize,
        geometry_data: usize,
        add_entry_name: usize,
        add_entry_xuid: usize,
    ) -> Self {
        Self {
            add_entry_name: Some(add_entry_name),
            add_entry_xuid: Some(add_entry_xuid),
            ..Self::new(entry_size, variant_index, add_entry_skin, skin_image, geometry_data)
        }
    }
}
