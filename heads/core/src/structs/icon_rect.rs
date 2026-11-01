/// An icon rectangle in screen units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl IconRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}
