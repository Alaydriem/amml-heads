#![allow(unsafe_code)]

use amml_msvc::{MsvcHeap, MsvcString, Optional};
use heads_core::structs::{ChatLine, MessageKind};

/// Mirror of the 26.60 `GuiMessage` (0x110 bytes), as its constructor leaves it.
#[repr(C)]
pub struct GuiMessageLayout {
    pub kind: i32,
    pub reserved0: u32,
    pub message: MsvcString,
    pub filtered_message: Optional<MsvcString>,
    pub tts_message: MsvcString,
    pub username: MsvcString,
    pub full_string: MsvcString,
    pub filtered_full_string: Optional<MsvcString>,
    pub author_xuid: MsvcString,
    pub force_visible: bool,
    pub tts_required: bool,
    pub reserved1: [u8; 2],
    pub lifetime: f32,
    pub reserved2: [u8; 0x10],
}

impl GuiMessageLayout {
    /// The line as the game built it, or `None` when one of its strings is not UTF-8.
    pub fn line(&self) -> Option<ChatLine<'_>> {
        Some(ChatLine {
            kind: MessageKind::from_raw(self.kind),
            full: self.full_string.to_str().ok()?,
            username: self.username.to_str().ok()?,
            author_xuid: self.author_xuid.to_str().ok()?,
        })
    }

    /// The line with the filtered full string, or `None` when that string has no value.
    pub fn filtered_line(&self) -> Option<ChatLine<'_>> {
        let filtered = self.filtered_full_string.get()?.to_str().ok()?;
        Some(ChatLine { full: filtered, ..self.line()? })
    }

    /// Replaces the full string with `text`. False, leaving the string as it was, when the heap gives no block.
    ///
    /// # Safety
    /// The full string must hold a block from `heap`, and no other thread may touch the message during the call.
    pub unsafe fn replace_full(&mut self, heap: &MsvcHeap, text: &str) -> bool {
        // SAFETY: the caller guarantees the string's block came from `heap`.
        unsafe { Self::replace(&mut self.full_string, heap, text) }
    }

    /// Replaces the filtered full string with `text`. False when it has no value or the heap gives no block.
    ///
    /// # Safety
    /// As `replace_full`, for the filtered full string.
    pub unsafe fn replace_filtered_full(&mut self, heap: &MsvcHeap, text: &str) -> bool {
        if self.filtered_full_string.get().is_none() {
            return false;
        }
        // SAFETY: `Optional` is repr(C) with the value at offset 0, and the value is engaged, so this is a live
        // string.
        let string = unsafe { &mut *std::ptr::addr_of_mut!(self.filtered_full_string).cast::<MsvcString>() };
        // SAFETY: the caller guarantees the string's block came from `heap`.
        unsafe { Self::replace(string, heap, text) }
    }

    // the new string is built before the old block is freed, so a failed allocation changes nothing
    unsafe fn replace(string: &mut MsvcString, heap: &MsvcHeap, text: &str) -> bool {
        // SAFETY: `heap` is the game operator new and delete.
        let Ok(new) = (unsafe { MsvcString::new_in(heap, text) }) else { return false };
        // SAFETY: the caller guarantees the old block came from `heap`; it is not used again.
        unsafe { string.release(heap) };
        *string = new;
        true
    }
}
