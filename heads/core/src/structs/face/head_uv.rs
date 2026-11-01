use super::UvRect;

/// Where the front of the head and the hat are, in a texture of `texture_width` x `texture_height` pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeadUv {
    pub face: UvRect,
    pub hat: Option<UvRect>,
    pub texture_width: u32,
    pub texture_height: u32,
}

impl HeadUv {
    pub fn new(face: UvRect, hat: Option<UvRect>, texture_width: u32, texture_height: u32) -> Self {
        Self { face, hat, texture_width, texture_height }
    }

    /// The head of the classic humanoid skin: face at (8,8) and hat at (40,8), 8x8 pixels each, in a 64x64 texture.
    pub fn classic() -> Self {
        Self::new(UvRect::new(8.0, 8.0, 8.0, 8.0), Some(UvRect::new(40.0, 8.0, 8.0, 8.0)), 64, 64)
    }

    /// The head of a persona animated face image: face at (8,8) and hat at (8,24), 8x8 pixels each, in the first
    /// 32x32 frame of a 32x64 texture.
    pub fn animated_face() -> Self {
        Self::new(UvRect::new(8.0, 8.0, 8.0, 8.0), Some(UvRect::new(8.0, 24.0, 8.0, 8.0)), 32, 64)
    }
}
