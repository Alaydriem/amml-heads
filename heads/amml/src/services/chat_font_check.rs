#![allow(unsafe_code)]

use super::MemoryReader;

type GetFontHandleFn = unsafe extern "C" fn(usize, *mut u64) -> *mut u64;
type FontHandleDtorFn = unsafe extern "C" fn(*mut u64);

/// Tells whether the default font of `ClientInstance::getFontHandle` is a `BitmapFont`, the class whose glyph
/// pages hold the face cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChatFontCheck {
    get_font_handle_slot: usize,
    font_handle_dtor: usize,
    bitmap_vtable: usize,
}

impl ChatFontCheck {
    /// `sizeof(FontHandle)`, the buffer `getFontHandle` writes through its hidden return pointer.
    pub const HANDLE_SIZE: usize = 0x50;
    /// `mDefaultFont` (a `shared_ptr<Font>`) inside `FontHandle`; its first word is the font.
    pub const DEFAULT_FONT: usize = 0x30;

    pub fn new(get_font_handle_slot: usize, font_handle_dtor: usize, bitmap_vtable: usize) -> Self {
        Self { get_font_handle_slot, font_handle_dtor, bitmap_vtable }
    }

    /// Whether the client's default font has the `BitmapFont` vtable. False for a null client or an empty slot.
    ///
    /// # Safety
    /// `client` must be null or the live `ClientInstance`, called on the game thread.
    pub unsafe fn is_bitmap(&self, client: usize) -> bool {
        // SAFETY: the caller guarantees the client is live; its first word is its vtable.
        let Some(vtable) = (unsafe { MemoryReader::read::<usize>(client) }) else { return false };
        let Some(slot) = self.get_font_handle_slot.checked_mul(8).and_then(|offset| vtable.checked_add(offset)) else {
            return false;
        };
        // SAFETY: the slot is inside the ClientInstance vtable, a table in the game image.
        let Some(code) = (unsafe { MemoryReader::read::<usize>(slot) }).filter(|code| *code != 0) else {
            return false;
        };
        let mut handle = [0u64; Self::HANDLE_SIZE / 8];
        // SAFETY: `code` is getFontHandle of the live client; it builds a FontHandle in the 0x50 byte buffer.
        unsafe { std::mem::transmute::<usize, GetFontHandleFn>(code)(client, handle.as_mut_ptr()) };
        let font = handle[Self::DEFAULT_FONT / 8] as usize;
        // SAFETY: a non-null default font is a live Font held by the handle; its first word is its vtable.
        let font_vtable = unsafe { MemoryReader::read::<usize>(font) };
        // SAFETY: the buffer holds the FontHandle just built; the destructor releases what it holds, once.
        unsafe { std::mem::transmute::<usize, FontHandleDtorFn>(self.font_handle_dtor)(handle.as_mut_ptr()) };
        font_vtable == Some(self.bitmap_vtable)
    }
}
