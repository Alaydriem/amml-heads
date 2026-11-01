use super::{FakeMemory, FakeSkin};

pub const ENTRY_SIZE: usize = 0xD8;
pub const VARIANT_INDEX: usize = 0xD0;
pub const ADD_ENTRY_SKIN: usize = 0xA8;
pub const ADD_ENTRY_NAME: usize = 0x20;
pub const ADD_ENTRY_XUID: usize = 0x40;

pub struct FakeEntry<'a> {
    pub variant: u8,
    pub uuid: [u8; 16],
    pub actor: i64,
    pub skin: Option<&'a FakeSkin>,
}

/// Fake `PlayerListPacket` and `PlayerSkinPacket` memory with the 26.60 layout.
pub struct FakePacket {
    packet: FakeMemory,
    // keeps the entries buffer alive for as long as the packet points at it
    _entries: FakeMemory,
}

impl FakePacket {
    pub fn player_list(entries: &[FakeEntry]) -> Self {
        Self::player_list_named(entries, &[])
    }

    /// As `player_list`, with each entry's name and XUID written as MSVC strings. A string of 16 bytes or more has
    /// its size and capacity set and no readable data.
    pub fn player_list_named(entries: &[FakeEntry], names: &[(&str, &str)]) -> Self {
        let mut buffer = FakeMemory::new(ENTRY_SIZE * entries.len().max(1));
        for (i, entry) in entries.iter().enumerate() {
            let base = i * ENTRY_SIZE;
            buffer.put(base + 0x08, entry.uuid);
            buffer.put(base + 0x18, entry.actor);
            if let Some(skin) = entry.skin {
                buffer.put(base + ADD_ENTRY_SKIN, skin.element());
            }
            buffer.put(base + VARIANT_INDEX, entry.variant);
            if let Some((name, xuid)) = names.get(i) {
                Self::put_string(&mut buffer, base + ADD_ENTRY_NAME, name);
                Self::put_string(&mut buffer, base + ADD_ENTRY_XUID, xuid);
            }
        }
        let first = buffer.addr();
        let last = first + ENTRY_SIZE * entries.len();
        let mut packet = FakeMemory::new(0x48);
        packet.put(0x30, first);
        packet.put(0x38, last);
        packet.put(0x40, last);
        Self { packet, _entries: buffer }
    }

    pub fn player_skin(uuid: [u8; 16], skin: &FakeSkin) -> Self {
        let mut packet = FakeMemory::new(0x98);
        packet.put(0x30, uuid);
        packet.put(0x40, skin.element());
        Self { packet, _entries: FakeMemory::new(8) }
    }

    fn put_string(buffer: &mut FakeMemory, at: usize, text: &str) {
        if text.len() <= 15 {
            let mut bytes = [0u8; 16];
            bytes[..text.len()].copy_from_slice(text.as_bytes());
            buffer.put(at, bytes);
            buffer.put(at + 0x18, 15usize);
        } else {
            buffer.put(at + 0x18, text.len());
        }
        buffer.put(at + 0x10, text.len());
    }

    pub fn addr(&self) -> usize {
        self.packet.addr()
    }
}
