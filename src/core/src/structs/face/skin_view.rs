/// A borrowed view of a skin image as the game stores it.
#[derive(Debug, Clone, Copy)]
pub struct SkinView<'a> {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub bytes: &'a [u8],
}

impl<'a> SkinView<'a> {
    pub fn new(width: u32, height: u32, format: u32, bytes: &'a [u8]) -> Self {
        Self { width, height, format, bytes }
    }
}
