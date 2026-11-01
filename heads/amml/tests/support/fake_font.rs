use std::cell::Cell;

use alaydriem_heads::services::ChatFontCheck;

use super::FakeMemory;

const GET_FONT_HANDLE_SLOT: usize = 2;

thread_local! {
    static DESTROYED: Cell<usize> = const { Cell::new(0) };
}

extern "C" fn get_font_handle(client: usize, out: *mut usize) -> *mut usize {
    // SAFETY: the fake client holds its default font at +8; `out` is a writable FontHandle buffer.
    unsafe {
        let font = ((client + 8) as *const usize).read();
        out.add(ChatFontCheck::DEFAULT_FONT / 8).write(font);
    }
    out
}

extern "C" fn font_handle_dtor(_handle: *mut usize) {
    DESTROYED.with(|count| count.set(count.get() + 1));
}

/// A fake `ClientInstance` whose `getFontHandle` gives a default font with the vtable `font_vtable`.
pub struct FakeFont {
    client: FakeMemory,
    _vtable: FakeMemory,
    _font: FakeMemory,
}

impl FakeFont {
    pub const BITMAP_VTABLE: usize = 0xF0EA8A0;
    pub const SMOOTH_VTABLE: usize = 0xF0E0000;

    pub fn new(font_vtable: usize) -> Self {
        let mut font = FakeMemory::new(0x10);
        font.put(0, font_vtable);
        let mut vtable = FakeMemory::new(8 * 8);
        vtable.put(GET_FONT_HANDLE_SLOT * 8, get_font_handle as *const () as usize);
        let mut client = FakeMemory::new(0x10);
        client.put(0, vtable.addr());
        client.put(8, font.addr());
        Self { client, _vtable: vtable, _font: font }
    }

    pub fn addr(&self) -> usize {
        self.client.addr()
    }

    pub fn check() -> ChatFontCheck {
        ChatFontCheck::new(GET_FONT_HANDLE_SLOT, font_handle_dtor as *const () as usize, Self::BITMAP_VTABLE)
    }

    /// How many font handles were destroyed on this thread.
    pub fn destroyed() -> usize {
        DESTROYED.with(Cell::get)
    }
}
