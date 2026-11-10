use crate::structs::{FaceImage, GlyphCell};

/// CPU copy of the 256x256 RGBA8 chat font page. Row 0 (16 pixel rows) holds the vanilla
/// glyphs; rows 1 to 15 hold one 16x16 face cell per player. A face is drawn at 10x10 (nearest
/// sampled) where the vanilla text-sized icons sit in their cells, so it is as tall as the text.
/// The page is marked dirty on every change so the render side uploads it once per change.
pub struct GlyphPage {
    pixels: Vec<u8>,
    dirty: bool,
}

impl GlyphPage {
    pub const SIDE: u32 = 256;
    pub const CELL_SIDE: u32 = 16;
    pub const FACE_SIDE: u32 = 10;
    // the vanilla hunger and armor glyphs fill rows 3 to 11 of their cells; the face sits on the same bottom row
    pub const FACE_TOP: u32 = 2;
    const ROW0_BYTES: usize = (Self::SIDE * Self::CELL_SIDE * 4) as usize;

    pub fn new(row0: &[u8]) -> Self {
        let mut pixels = vec![0u8; (Self::SIDE * Self::SIDE * 4) as usize];
        if row0.len() == Self::ROW0_BYTES {
            pixels[..Self::ROW0_BYTES].copy_from_slice(row0);
        }
        Self { pixels, dirty: true }
    }

    pub fn write_face(&mut self, cell: GlyphCell, face: &FaceImage) {
        let cell_x = u32::from(cell.0 % 16) * Self::CELL_SIDE;
        let face_y = (u32::from(cell.0 / 16) + 1) * Self::CELL_SIDE + Self::FACE_TOP;
        for y in 0..Self::FACE_SIDE {
            for x in 0..Self::FACE_SIDE {
                let at = (((face_y + y) * Self::SIDE + cell_x + x) * 4) as usize;
                let sample = face.pixel(x * FaceImage::SIDE / Self::FACE_SIDE, y * FaceImage::SIDE / Self::FACE_SIDE);
                self.pixels[at..at + 4].copy_from_slice(&sample);
            }
        }
        self.dirty = true;
    }

    /// Marks the page changed again, after an upload that did not happen.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn take_dirty(&mut self) -> Option<&[u8]> {
        if self.dirty {
            self.dirty = false;
            Some(&self.pixels)
        } else {
            None
        }
    }
}
