/// The icon character of each player that has a face icon.
///
/// Bound: at most `CAPACITY` (240) players, the number of inline icons the loader serves. When full, `set` refuses a
/// new player and the player gets no head. `clear` empties it, as the loader frees every icon at a world leave.
pub struct FaceGlyphs {
    entries: Vec<([u8; 16], char)>,
    full_seen: bool,
    full_reported: bool,
}

impl FaceGlyphs {
    pub const CAPACITY: usize = 240;

    pub fn new() -> Self {
        Self { entries: Vec::with_capacity(Self::CAPACITY), full_seen: false, full_reported: false }
    }

    /// Gives `uuid` the character `glyph`, replacing an earlier one. False when the player is new and the table is
    /// full.
    pub fn set(&mut self, uuid: [u8; 16], glyph: char) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.0 == uuid) {
            entry.1 = glyph;
            return true;
        }
        if self.entries.len() >= Self::CAPACITY {
            self.full_seen = true;
            return false;
        }
        self.entries.push((uuid, glyph));
        true
    }

    pub fn glyph_of(&self, uuid: [u8; 16]) -> Option<char> {
        self.entries.iter().find(|entry| entry.0 == uuid).map(|entry| entry.1)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.full_seen = false;
        self.full_reported = false;
    }

    /// True once, after a `set` was refused because the table is full.
    pub fn full_reported(&mut self) -> bool {
        if self.full_seen && !self.full_reported {
            self.full_reported = true;
            return true;
        }
        false
    }
}

impl Default for FaceGlyphs {
    fn default() -> Self {
        Self::new()
    }
}
