/// One face cell on the chat glyph page. Cell `i` is the private-use code point U+E110 + `i`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlyphCell(pub u8);

impl GlyphCell {
    pub const FIRST_CODEPOINT: u32 = 0xE110;
    pub const LAST_CODEPOINT: u32 = 0xE1FF;
    /// The width the font gives a cell that holds a face: the 10 pixel face plus one pixel of space, as the font
    /// measures the vanilla text-sized glyphs (9 pixels wide, width 10).
    pub const WIDTH: f32 = 11.0;

    pub fn codepoint(self) -> char {
        char::from_u32(Self::FIRST_CODEPOINT + u32::from(self.0)).unwrap_or(char::REPLACEMENT_CHARACTER)
    }

    pub fn from_codepoint(c: u32) -> Option<GlyphCell> {
        if (Self::FIRST_CODEPOINT..=Self::LAST_CODEPOINT).contains(&c) {
            Some(GlyphCell((c - Self::FIRST_CODEPOINT) as u8))
        } else {
            None
        }
    }
}
