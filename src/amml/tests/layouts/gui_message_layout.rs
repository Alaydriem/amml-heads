use std::mem::{offset_of, size_of};

use alaydriem_heads::layouts::GuiMessageLayout;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(GuiMessageLayout, kind), 0x00);
    assert_eq!(offset_of!(GuiMessageLayout, message), 0x08);
    assert_eq!(offset_of!(GuiMessageLayout, filtered_message), 0x28);
    assert_eq!(offset_of!(GuiMessageLayout, tts_message), 0x50);
    assert_eq!(offset_of!(GuiMessageLayout, username), 0x70);
    assert_eq!(offset_of!(GuiMessageLayout, full_string), 0x90);
    assert_eq!(offset_of!(GuiMessageLayout, filtered_full_string), 0xB0);
    assert_eq!(offset_of!(GuiMessageLayout, author_xuid), 0xD8);
    assert_eq!(offset_of!(GuiMessageLayout, force_visible), 0xF8);
    assert_eq!(offset_of!(GuiMessageLayout, tts_required), 0xF9);
    assert_eq!(offset_of!(GuiMessageLayout, lifetime), 0xFC);
    assert_eq!(size_of::<GuiMessageLayout>(), 0x110);
}
