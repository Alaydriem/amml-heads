use std::mem::{offset_of, size_of};

use alaydriem_heads::layouts::ImageLayout;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(ImageLayout, format), 0x00);
    assert_eq!(offset_of!(ImageLayout, width), 0x04);
    assert_eq!(offset_of!(ImageLayout, height), 0x08);
    assert_eq!(offset_of!(ImageLayout, depth), 0x0C);
    assert_eq!(offset_of!(ImageLayout, usage), 0x10);
    assert_eq!(offset_of!(ImageLayout, alpha_usage), 0x11);
    assert_eq!(offset_of!(ImageLayout, blob_deleter), 0x18);
    assert_eq!(offset_of!(ImageLayout, blob_data), 0x20);
    assert_eq!(offset_of!(ImageLayout, blob_size), 0x28);
    assert_eq!(size_of::<ImageLayout>(), 0x30);
}
