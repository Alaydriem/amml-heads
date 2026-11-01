/// A 32x32 RGBA8 player face, row-major.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceImage {
    pub pixels: [u8; 4096],
}

impl FaceImage {
    pub const SIDE: u32 = 32;

    pub fn new(pixels: [u8; 4096]) -> Self {
        Self { pixels }
    }

    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let at = ((y * Self::SIDE + x) * 4) as usize;
        [self.pixels[at], self.pixels[at + 1], self.pixels[at + 2], self.pixels[at + 3]]
    }
}
