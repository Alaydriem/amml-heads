use alaydriem_heads::services::FaceBoard;
use amml_api::TextureId;
use heads_core::structs::{FaceImage, FaceUpdate};

fn add(board: &mut FaceBoard, n: u8) {
    let face = Box::new(FaceImage::new([n; 4096]));
    board.apply(FaceUpdate::Add { uuid: [n; 16], actor: i64::from(n), face });
}

fn upload_all(board: &mut FaceBoard) {
    let mut jobs = Vec::new();
    board.take_pending(&mut jobs);
    for (key, _) in jobs {
        let texture = TextureId { id: key.actor as u64 + 1, generation: 0 };
        assert_eq!(board.attach(key, texture), None);
    }
}

#[test]
fn a_face_gets_its_texture_once_uploaded() {
    let mut board = FaceBoard::new();
    add(&mut board, 7);
    assert!(board.has_pending_upload());
    assert_eq!(board.texture_for(7), None);

    upload_all(&mut board);

    assert!(!board.has_pending_upload());
    assert_eq!(board.texture_for(7), Some(TextureId { id: 8, generation: 0 }));
}

#[test]
fn the_board_stops_at_128_faces_and_hands_the_oldest_texture_back() {
    let mut board = FaceBoard::new();
    for n in 0..FaceBoard::CAPACITY as u8 {
        add(&mut board, n);
    }
    upload_all(&mut board);
    assert!(board.take_released().is_empty());

    add(&mut board, 200);

    assert_eq!(board.len(), FaceBoard::CAPACITY);
    assert_eq!(board.take_released(), vec![TextureId { id: 1, generation: 0 }]);
    assert_eq!(board.texture_for(0), None);
}

#[test]
fn a_new_skin_hands_the_old_texture_back_and_waits_for_an_upload() {
    let mut board = FaceBoard::new();
    add(&mut board, 3);
    upload_all(&mut board);

    board.apply(FaceUpdate::Skin { uuid: [3; 16], face: Box::new(FaceImage::new([9; 4096])) });

    assert_eq!(board.take_released(), vec![TextureId { id: 4, generation: 0 }]);
    assert!(board.has_pending_upload());
}

#[test]
fn a_texture_for_a_face_that_went_comes_back() {
    let mut board = FaceBoard::new();
    add(&mut board, 3);
    let mut jobs = Vec::new();
    board.take_pending(&mut jobs);
    board.apply(FaceUpdate::Remove { uuid: [3; 16] });

    let stale = board.attach(jobs[0].0, TextureId { id: 99, generation: 0 });

    assert_eq!(stale, Some(TextureId { id: 99, generation: 0 }));
}

#[test]
fn clear_releases_every_texture() {
    let mut board = FaceBoard::new();
    for n in 0..5 {
        add(&mut board, n);
    }
    upload_all(&mut board);

    board.apply(FaceUpdate::Clear);

    assert!(board.is_empty());
    assert_eq!(board.take_released().len(), 5);
}

#[test]
fn a_failed_upload_is_offered_again() {
    let mut board = FaceBoard::new();
    add(&mut board, 1);
    let mut jobs = Vec::new();
    board.take_pending(&mut jobs);
    assert!(!board.has_pending_upload());

    board.retry_upload(jobs[0].0);

    assert!(board.has_pending_upload());
}
