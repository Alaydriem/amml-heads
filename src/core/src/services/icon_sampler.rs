use crate::structs::{FaceImage, IconImage};

/// Draws a face into an inline icon image: 10x10 (nearest sampled) on the bottom row where the vanilla text-sized
/// icons sit in their 16x16 cell, so it is as tall as the text.
pub struct IconSampler;

impl IconSampler {
    pub const FACE_SIDE: u32 = 10;
    // the vanilla hunger and armor glyphs fill rows 3 to 11 of their cells; the face sits on the same bottom row
    pub const FACE_TOP: u32 = 2;

    pub fn sample(face: &FaceImage) -> IconImage {
        let mut pixels = [0u8; 1024];
        for y in 0..Self::FACE_SIDE {
            for x in 0..Self::FACE_SIDE {
                let at = (((Self::FACE_TOP + y) * IconImage::SIDE + x) * 4) as usize;
                let sample = face.pixel(x * FaceImage::SIDE / Self::FACE_SIDE, y * FaceImage::SIDE / Self::FACE_SIDE);
                pixels[at..at + 4].copy_from_slice(&sample);
            }
        }
        IconImage::new(pixels)
    }
}
