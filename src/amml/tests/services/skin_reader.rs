use alaydriem_heads::services::SkinReader;
use alaydriem_heads::structs::PacketOffsets;

use crate::support::{FakeLayout, FakeMemory, FakeSkin};

fn offsets() -> PacketOffsets {
    FakeLayout::packet_offsets()
}

#[test]
fn reads_rgba_skin_through_ref() {
    let skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    // SAFETY: the fake skin lives past the view and its layout matches the offsets.
    let view = unsafe { SkinReader::view(skin.skin_ref(), &offsets()) }.unwrap();
    assert_eq!((view.width, view.height, view.format), (64, 64, 4));
    assert_eq!(view.bytes, skin.blob());
}

#[test]
fn null_ref_gives_none() {
    // SAFETY: a null reference is rejected before any read.
    assert!(unsafe { SkinReader::view(0, &offsets()) }.is_none());

    let empty = FakeMemory::new(16);
    // SAFETY: the buffer holds a null element pointer, which is rejected.
    assert!(unsafe { SkinReader::view(empty.addr(), &offsets()) }.is_none());
}

#[test]
fn image_size_mismatch_gives_none() {
    let skin = FakeSkin::new(64, 64, 4, 100, [1, 2, 3, 4]);
    // SAFETY: the fake skin lives for the call and its layout matches the offsets.
    assert!(unsafe { SkinReader::view(skin.skin_ref(), &offsets()) }.is_none());
}

#[test]
fn oversized_dimensions_give_none() {
    let skin = FakeSkin::new(u32::MAX, u32::MAX, 4, 64, [1, 2, 3, 4]);
    // SAFETY: the fake skin lives for the call and its layout matches the offsets.
    assert!(unsafe { SkinReader::view(skin.skin_ref(), &offsets()) }.is_none());
}
