use alaydriem_heads::structs::TextureLayout;

use crate::support::FakeLayout;

fn layout(buffer_size: usize, deleter: usize, data: usize, size: usize, location: usize) -> Option<TextureLayout> {
    TextureLayout::new(FakeLayout::TEXTURE_GROUP_BASE, buffer_size, deleter, data, size, location)
}

#[test]
fn the_2660_layout_gives_the_blob_words() {
    let layout = FakeLayout::texture_layout();

    assert_eq!(layout.group_base(), 0x18);
    assert_eq!(layout.buffer_words(), 6);
    assert_eq!((layout.deleter_word(), layout.data_word(), layout.size_word()), (0, 1, 2));
}

#[test]
fn an_image_buffer_larger_than_the_stack_buffer_is_refused() {
    let words = TextureLayout::BUFFER_CAPACITY_WORDS;

    assert!(layout(words * 8, 0, 8, 0x10, 0x38).is_some());
    assert!(layout((words + 1) * 8, 0, 8, 0x10, 0x38).is_none());
}

#[test]
fn a_blob_member_outside_the_buffer_or_off_a_word_is_refused() {
    assert!(layout(0x30, 0, 0x30, 0x10, 0x38).is_none());
    assert!(layout(0x30, 0, 8, 0x12, 0x38).is_none());
    assert!(layout(0x2C, 0, 8, 0x10, 0x38).is_none());
    assert!(layout(0, 0, 0, 0, 0x38).is_none());
}

#[test]
fn a_resource_location_size_other_than_the_mirror_is_refused() {
    assert!(layout(0x30, 0, 8, 0x10, 0x40).is_none());
    assert!(layout(0x30, 0, 8, 0x10, 0x30).is_none());
}
