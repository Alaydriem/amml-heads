use std::mem::{offset_of, size_of};

use alaydriem_heads::layouts::ResourceLocationLayout;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(ResourceLocationLayout, file_system), 0x00);
    assert_eq!(offset_of!(ResourceLocationLayout, path), 0x08);
    assert_eq!(offset_of!(ResourceLocationLayout, path_hash), 0x28);
    assert_eq!(offset_of!(ResourceLocationLayout, full_hash), 0x30);
    assert_eq!(size_of::<ResourceLocationLayout>(), 0x38);
}
