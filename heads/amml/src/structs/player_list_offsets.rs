/// The layout of a `PlayerListPacket` and of one of its entries, read from the field entries when the mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerListOffsets {
    /// `mEntries` (a `std::vector`) inside the packet.
    pub entries: usize,
    /// `sizeof(PlayerListPacket::Entry)`, the stride of the entries vector.
    pub entry_size: usize,
    /// The byte in an entry that selects Remove (0) or Add (1).
    pub variant_index: usize,
    /// `mUUID` inside an entry; both variants keep it here.
    pub uuid: usize,
    /// `mId` (an `ActorUniqueID`) inside an Add entry.
    pub actor: usize,
    /// `mSkin` (a `shared_ptr`) inside an Add entry.
    pub skin: usize,
}

impl PlayerListOffsets {
    /// The `std::variant` alternative indices of an entry, in the declared order `<RemoveEntry, AddEntry>`.
    pub const VARIANT_REMOVE: u8 = 0;
    pub const VARIANT_ADD: u8 = 1;

    pub fn new(
        entries: usize,
        entry_size: usize,
        variant_index: usize,
        uuid: usize,
        actor: usize,
        skin: usize,
    ) -> Self {
        Self { entries, entry_size, variant_index, uuid, actor, skin }
    }
}
