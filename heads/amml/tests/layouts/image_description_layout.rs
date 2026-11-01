use std::mem::offset_of;

use alaydriem_heads::layouts::ImageDescriptionLayout;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(ImageDescriptionLayout, width), 0x00);
    assert_eq!(offset_of!(ImageDescriptionLayout, height), 0x04);
    assert_eq!(offset_of!(ImageDescriptionLayout, format), 0x08);
    assert_eq!(offset_of!(ImageDescriptionLayout, color_space), 0x0C);
    assert_eq!(offset_of!(ImageDescriptionLayout, image_type), 0x0D);
    assert_eq!(offset_of!(ImageDescriptionLayout, alpha_usage), 0x0E);
    assert_eq!(offset_of!(ImageDescriptionLayout, depth), 0x10);
}

#[test]
fn face_description_is_rgba8_32x32() {
    let description = ImageDescriptionLayout::face(32);
    assert_eq!((description.width, description.height), (32, 32));
    assert_eq!(description.format, 0x1C);
    assert_eq!(description.color_space, 1);
    assert_eq!(description.depth, 1);
}
