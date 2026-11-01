use amml_sdk::amml_detour;

use heads_core::structs::GlyphCell;

use crate::services::ChatRuntime;

// float BitmapFont::_getCharWidth(int character, bool forceUnicode), called for each character the game measures.
// A face cell is as wide as the face; every other character goes to the original
amml_detour!(pub mod char_width(font: usize, character: i32, force_unicode: u8) -> f32 |original| {
    if ChatRuntime::has_face(character as u32) {
        return GlyphCell::WIDTH;
    }
    original(font, character, force_unicode)
});

pub use char_width::{GUARD, detour};
