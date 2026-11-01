use std::mem::offset_of;

use alaydriem_heads::layouts::ListNodeLayout;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(ListNodeLayout, next), 0x00);
    assert_eq!(offset_of!(ListNodeLayout, key), 0x10);
    assert_eq!(offset_of!(ListNodeLayout, waypoint), 0x20);
}
