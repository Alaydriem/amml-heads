use alaydriem_heads::services::TextureBridge;

use crate::support::{FakeGame, FakeLayout, FakeRig, GameCall};

const FACE_PIXEL: [u8; 4] = [9, 8, 7, 6];

fn bridge() -> TextureBridge {
    FakeGame::take_log();
    TextureBridge::new(FakeGame::texture_fns(), FakeLayout::texture_layout())
}

#[test]
fn upload_names_a_pool_slot_and_reads_the_texture_group_offset() {
    let mut bridge = bridge();
    let face = FakeRig::face(FACE_PIXEL);

    let texture = bridge.upload(FakeGame::GROUP, &face).unwrap();

    assert_eq!(texture.slot(), 0);
    assert!(!texture.texture_ptr().is_null());
    assert_eq!(
        FakeGame::take_log(),
        vec![
            GameCall::Upload { group: FakeGame::GROUP, slot: 0, pixel: FACE_PIXEL },
            GameCall::Get { group: FakeGame::GROUP + FakeGame::GET_GROUP_OFFSET, slot: 0 },
        ]
    );
    let second = bridge.upload(FakeGame::GROUP, &face).unwrap();
    assert_eq!(second.slot(), 1);
}

#[test]
fn unload_unloads_the_name_and_gives_it_back() {
    let mut bridge = bridge();
    let face = FakeRig::face(FACE_PIXEL);
    let texture = bridge.upload(FakeGame::GROUP, &face).unwrap();
    assert_eq!(bridge.free_names(), TextureBridge::POOL - 1);
    FakeGame::take_log();

    bridge.unload(FakeGame::GROUP, texture);

    assert_eq!(FakeGame::take_log(), vec![GameCall::Unload { group: FakeGame::GROUP, slot: 0 }]);
    assert_eq!(bridge.free_names(), TextureBridge::POOL);
}

#[test]
fn failed_get_unloads_and_returns_none() {
    let mut bridge = bridge();
    let face = FakeRig::face(FACE_PIXEL);
    FakeGame::fail_get(true);

    assert!(bridge.upload(FakeGame::GROUP, &face).is_none());

    assert!(FakeGame::take_log().contains(&GameCall::Unload { group: FakeGame::GROUP, slot: 0 }));
    assert_eq!(bridge.free_names(), TextureBridge::POOL);
}

#[test]
fn wrong_size_buffer_is_not_uploaded() {
    let mut bridge = bridge();
    let face = FakeRig::face(FACE_PIXEL);
    FakeGame::bad_buffer(true);

    assert!(bridge.upload(FakeGame::GROUP, &face).is_none());

    assert!(FakeGame::take_log().is_empty());
    assert_eq!(bridge.free_names(), TextureBridge::POOL);
}

#[test]
fn empty_name_pool_returns_none() {
    let mut bridge = bridge();
    let face = FakeRig::face(FACE_PIXEL);
    let held: Vec<_> = (0..TextureBridge::POOL).map(|_| bridge.upload(FakeGame::GROUP, &face).unwrap()).collect();

    assert_eq!(bridge.free_names(), 0);
    assert!(bridge.upload(FakeGame::GROUP, &face).is_none());
    assert_eq!(held.len(), TextureBridge::POOL);
}

#[test]
fn upload_page_builds_one_location_and_uploads_256_square() {
    let mut bridge = bridge();
    let mut page = vec![0u8; 256 * 256 * 4];
    page[..4].copy_from_slice(&[5, 6, 7, 8]);
    let before = FakeGame::locations_made();

    assert!(bridge.upload_page(FakeGame::GROUP, &page));
    assert!(bridge.upload_page(FakeGame::GROUP, &page));

    assert_eq!(FakeGame::locations_made() - before, 1);
    let upload = GameCall::PageUpload { group: FakeGame::GROUP, bytes: 256 * 256 * 4, pixel: [5, 6, 7, 8] };
    assert_eq!(FakeGame::take_log(), vec![upload.clone(), upload]);
    assert_eq!(bridge.free_names(), TextureBridge::POOL);
}

#[test]
fn upload_page_refuses_pixels_of_the_wrong_size() {
    let mut bridge = bridge();

    assert!(!bridge.upload_page(FakeGame::GROUP, &[0u8; 4096]));

    assert!(FakeGame::take_log().is_empty());
}
