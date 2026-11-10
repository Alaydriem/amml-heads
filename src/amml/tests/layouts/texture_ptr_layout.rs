use std::mem::{offset_of, size_of};

use alaydriem_heads::layouts::TexturePtrLayout;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(TexturePtrLayout, texture), 0x00);
    assert_eq!(offset_of!(TexturePtrLayout, location), 0x10);
    assert_eq!(size_of::<TexturePtrLayout>(), 0x20);
}
