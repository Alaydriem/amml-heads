use super::{PlayerListOffsets, SkinOffsets, SkinPacketOffsets};

/// The packet and skin layouts the packet handlers read, from the field entries when the mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketOffsets {
    pub list: PlayerListOffsets,
    pub skin_packet: SkinPacketOffsets,
    pub skin: SkinOffsets,
    /// `mName` inside an Add entry; `None` when chat heads is off.
    pub add_entry_name: Option<usize>,
    /// `mXUID` inside an Add entry; `None` when chat heads is off.
    pub add_entry_xuid: Option<usize>,
}

impl PacketOffsets {
    pub fn new(list: PlayerListOffsets, skin_packet: SkinPacketOffsets, skin: SkinOffsets) -> Self {
        Self { list, skin_packet, skin, add_entry_name: None, add_entry_xuid: None }
    }

    /// As `new`, with the Add entry name and XUID offsets that the chat feed reads.
    pub fn new_with_chat(
        list: PlayerListOffsets,
        skin_packet: SkinPacketOffsets,
        skin: SkinOffsets,
        add_entry_name: usize,
        add_entry_xuid: usize,
    ) -> Self {
        Self {
            add_entry_name: Some(add_entry_name),
            add_entry_xuid: Some(add_entry_xuid),
            ..Self::new(list, skin_packet, skin)
        }
    }
}
