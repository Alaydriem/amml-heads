use alaydriem_heads::services::ChatFontCheck;

use crate::support::FakeFont;

#[test]
fn a_bitmap_default_font_passes_and_the_handle_is_destroyed() {
    let bitmap = FakeFont::new(FakeFont::BITMAP_VTABLE);
    let smooth = FakeFont::new(FakeFont::SMOOTH_VTABLE);
    let check = FakeFont::check();

    // SAFETY: the fake clients live for the calls.
    unsafe {
        assert!(check.is_bitmap(bitmap.addr()));
        assert!(!check.is_bitmap(smooth.addr()));
        assert!(!check.is_bitmap(0));
    }
    assert_eq!(FakeFont::destroyed(), 2);
}

#[test]
fn a_font_handle_layout_that_does_not_fit_the_stack_buffer_is_refused() {
    let words = ChatFontCheck::HANDLE_CAPACITY_WORDS;

    assert!(ChatFontCheck::new(2, 0x1000, 0x2000, words * 8, 0x30).is_some());
    assert!(ChatFontCheck::new(2, 0x1000, 0x2000, (words + 1) * 8, 0x30).is_none());
    assert!(ChatFontCheck::new(2, 0x1000, 0x2000, 0x50, 0x50).is_none());
    assert!(ChatFontCheck::new(2, 0x1000, 0x2000, 0x50, 0x34).is_none());
    assert!(ChatFontCheck::new(2, 0x1000, 0x2000, 0x4C, 0x30).is_none());
    assert!(ChatFontCheck::new(2, 0x1000, 0x2000, 0, 0).is_none());
}
