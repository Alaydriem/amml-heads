use std::ffi::c_void;
use std::sync::atomic::Ordering;

use alaydriem_heads::detours::char_width;
use alaydriem_heads::services::{ChatRuntime, ChatState};
use heads_core::services::GlyphPage;

use crate::support::FakeRig;

extern "C" fn fake_original(_font: usize, _character: i32, _force_unicode: u8) -> f32 {
    7.0
}

#[test]
fn width_is_11_for_a_cell_with_a_face_and_the_original_otherwise() {
    char_width::GUARD.original().store(fake_original as *const () as *mut c_void, Ordering::Release);
    ChatRuntime::install(ChatState::new(GlyphPage::new(&[]), ChatRuntime::flags()), ChatRuntime::flags());
    ChatRuntime::with_state_blocking(|state| state.on_face([1; 16], "1", "Alex", &FakeRig::face([1, 1, 1, 255])));

    assert_eq!(char_width::detour(0x10, 0xE110, 0), 11.0);
    assert_eq!(char_width::detour(0x10, 0xE111, 0), 7.0);
    assert_eq!(char_width::detour(0x10, 'A' as i32, 0), 7.0);
    assert_eq!(char_width::detour(0x10, 0xE101, 1), 7.0);
}
