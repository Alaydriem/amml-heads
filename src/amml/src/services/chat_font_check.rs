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
    default_font_word: usize,
}

impl ChatFontCheck {
    /// The most words of `FontHandle` the check builds on its stack.
    pub const HANDLE_CAPACITY_WORDS: usize = 32;
    const WORD: usize = size_of::<u64>();

    /// `handle_size` is `sizeof(FontHandle)`, the buffer `getFontHandle` writes through its hidden return pointer,
    /// and `default_font` is `mDefaultFont` (a `shared_ptr<Font>`) inside it. `None` when the handle is larger than
    /// `HANDLE_CAPACITY_WORDS` words or the default font is not a word inside it.
    pub fn new(
        get_font_handle_slot: usize,
        font_handle_dtor: usize,
        bitmap_vtable: usize,
        handle_size: usize,
        default_font: usize,
    ) -> Option<Self> {
        if handle_size == 0 || !handle_size.is_multiple_of(Self::WORD) || !default_font.is_multiple_of(Self::WORD) {
            return None;
        }
        let handle_words = handle_size / Self::WORD;
        let default_font_word = default_font / Self::WORD;
        if handle_words > Self::HANDLE_CAPACITY_WORDS || default_font_word >= handle_words {
            return None;
        }
        Some(Self { get_font_handle_slot, font_handle_dtor, bitmap_vtable, default_font_word })
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
        let mut handle = [0u64; Self::HANDLE_CAPACITY_WORDS];
        // SAFETY: `code` is getFontHandle of the live client; it builds a FontHandle of the entry size, which `new`
        // holds within the buffer.
        unsafe { std::mem::transmute::<usize, GetFontHandleFn>(code)(client, handle.as_mut_ptr()) };
        let font = handle[self.default_font_word] as usize;
        // SAFETY: a non-null default font is a live Font held by the handle; its first word is its vtable.
        let font_vtable = unsafe { MemoryReader::read::<usize>(font) };
        // SAFETY: the buffer holds the FontHandle just built; the destructor releases what it holds, once.
        unsafe { std::mem::transmute::<usize, FontHandleDtorFn>(self.font_handle_dtor)(handle.as_mut_ptr()) };
        font_vtable == Some(self.bitmap_vtable)
    }
}
