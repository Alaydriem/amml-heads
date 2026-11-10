/// The layout of a `PlayerSkinPacket`, read from the field entries when the mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkinPacketOffsets {
    /// `mUUID` inside the packet.
    pub uuid: usize,
    /// `mSkin` (a `SerializedSkinRef`) inside the packet.
    pub skin: usize,
}

impl SkinPacketOffsets {
    pub fn new(uuid: usize, skin: usize) -> Self {
        Self { uuid, skin }
    }
}
