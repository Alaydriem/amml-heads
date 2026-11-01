use std::sync::atomic::{AtomicU64, Ordering};

use crate::structs::GlyphCell;

/// One bit per glyph cell, set once the cell holds a face. Fixed size: 4 words cover the 240 cells.
/// A flag is never cleared. Reads take no lock and do not allocate.
pub struct CellFlags {
    words: [AtomicU64; 4],
}

impl CellFlags {
    pub fn new() -> Self {
        Self { words: [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)] }
    }

    pub fn set(&self, cell: GlyphCell) {
        let index = usize::from(cell.0);
        self.words[index / 64].fetch_or(1 << (index % 64), Ordering::Relaxed);
    }

    pub fn has_face(&self, codepoint: u32) -> bool {
        match GlyphCell::from_codepoint(codepoint) {
            Some(cell) => {
                let index = usize::from(cell.0);
                self.words[index / 64].load(Ordering::Relaxed) & (1 << (index % 64)) != 0
            }
            None => false,
        }
    }
}

impl Default for CellFlags {
    fn default() -> Self {
        Self::new()
    }
}
