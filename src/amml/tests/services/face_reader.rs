use std::sync::{Arc, Mutex};

use alaydriem_heads::services::FaceReader;
use alaydriem_heads::structs::FaceRead;
use amml_api::amml_abi::PlayerSkinChanged;
use amml_api::testing::FakeSkin;
use amml_api::{HookOrder, PlayerSkinChangedExt};

use crate::support::heads_rig::HeadsRig;
use crate::support::skins::Skins;

// the read of `skin` inside a skin change event; the result is used after the event is over
fn read(skin: FakeSkin) -> FaceRead {
    let game = HeadsRig::served_game();
    let seen: Arc<Mutex<Option<FaceRead>>> = Arc::new(Mutex::new(None));
    let sink = seen.clone();
    let _listener = game
        .ctx()
        .events()
        .listen::<PlayerSkinChanged>(HookOrder::NORMAL, move |event, _| {
            *sink.lock().unwrap() = event.skin().map(|skin| FaceReader::read(&skin));
        })
        .unwrap();
    game.raise_skin(HeadsRig::STEVE, Some(skin));
    let read = seen.lock().unwrap().take();
    read.unwrap()
}

#[test]
fn a_classic_skin_gives_its_face_at_the_classic_place() {
    let read = read(Skins::classic([10, 200, 30]));

    let face = read.face.unwrap();
    assert_eq!(face.pixel(0, 0), [10, 200, 30, 255]);
    assert_eq!(face.pixel(31, 31), [10, 200, 30, 255]);
    assert_eq!((read.source, read.head_source, read.outcome.as_str()), ("skin", "classic", "Ok"));
}

#[test]
fn a_geometry_that_moves_the_head_moves_the_face() {
    let read = read(Skins::geometry([0, 255, 0]));

    assert_eq!(read.face.unwrap().pixel(5, 5), [0, 255, 0, 255]);
    assert_eq!(read.head_source, "geometry");
    assert_eq!(read.geometry_json_len, Some(Skins::CLASSIC_112.len()));
}

#[test]
fn a_persona_skin_gives_the_animated_face() {
    let read = read(Skins::persona([0, 0, 255]));

    assert_eq!(read.face.unwrap().pixel(16, 16), [0, 0, 255, 255]);
    assert_eq!((read.source, read.head_source), ("persona", "animated_face"));
}

#[test]
fn an_animated_image_of_a_skin_that_is_not_persona_is_not_used() {
    let mut skin = Skins::persona([0, 0, 255]);
    skin.persona = false;

    let read = read(skin);

    assert_eq!(read.face.unwrap().pixel(16, 16), [255, 0, 0, 255]);
    assert_eq!(read.source, "skin");
}

#[test]
fn a_skin_the_loader_gives_no_image_for_gives_no_face() {
    let read = read(Skins::unreadable());

    assert!(read.face.is_none());
    assert_eq!(read.outcome, "SkinNotReadable");
}

#[test]
fn a_skin_with_no_visible_face_gives_no_face() {
    let read = read(Skins::empty());

    assert!(read.face.is_none());
    assert_eq!(read.outcome, "EmptyFace");
}
