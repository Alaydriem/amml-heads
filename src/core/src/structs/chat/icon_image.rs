/// A 16x16 RGBA8 inline icon image, row-major: a player face in the top left of the cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconImage {
    pub pixels: [u8; 1024],
}

impl IconImage {
    pub const SIDE: u32 = 16;

    pub fn new(pixels: [u8; 1024]) -> Self {
        Self { pixels }
    }

    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let at = ((y * Self::SIDE + x) * 4) as usize;
        [self.pixels[at], self.pixels[at + 1], self.pixels[at + 2], self.pixels[at + 3]]
    }
}
