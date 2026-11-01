/// A waypoint colour as the game stores it, one float per lane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TagColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl TagColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
