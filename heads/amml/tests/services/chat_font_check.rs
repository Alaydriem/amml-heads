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
