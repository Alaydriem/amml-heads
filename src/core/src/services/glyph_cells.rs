use std::sync::Arc;

use crate::services::CellFlags;
use crate::structs::GlyphCell;

/// Assigns each player UUID a fixed cell on the chat glyph page.
///
/// Bound: at most `CAPACITY` (240) cells for the life of the process. There is no eviction: a
/// cell, once given, never moves, so chat lines already written keep the right face. When the
/// cells are full, `cell_for` returns `None` and the player gets no head.
pub struct GlyphCells {
    uuids: Vec<[u8; 16]>,
    flags: Arc<CellFlags>,
    full_seen: bool,
    full_reported: bool,
}

impl GlyphCells {
    pub const CAPACITY: usize = 240;

    pub fn new(flags: Arc<CellFlags>) -> Self {
        Self { uuids: Vec::with_capacity(Self::CAPACITY), flags, full_seen: false, full_reported: false }
    }

    pub fn cell_for(&mut self, uuid: [u8; 16]) -> Option<GlyphCell> {
        if let Some(cell) = self.cell_of(uuid) {
            return Some(cell);
        }
        if self.uuids.len() >= Self::CAPACITY {
            self.full_seen = true;
            return None;
        }
        self.uuids.push(uuid);
        Some(GlyphCell((self.uuids.len() - 1) as u8))
    }

    pub fn cell_of(&self, uuid: [u8; 16]) -> Option<GlyphCell> {
        self.uuids.iter().position(|known| *known == uuid).map(|index| GlyphCell(index as u8))
    }

    pub fn mark_face(&self, cell: GlyphCell) {
        self.flags.set(cell);
    }

    pub fn has_face(&self, codepoint: u32) -> bool {
        self.flags.has_face(codepoint)
    }

    pub fn full_reported(&mut self) -> bool {
        if self.full_seen && !self.full_reported {
            self.full_reported = true;
            return true;
        }
        false
    }
}
