use alaydriem_heads::services::{GeometryReader, PacketObserver, SkinDiagnostics};
use alaydriem_heads::structs::PacketOffsets;
use crossbeam_channel::bounded;
use heads_core::structs::{FaceUpdate, HeadUv, UvRect};

use crate::support::{FakeEntry, FakeGeometry, FakeLayout, FakePacket, FakeSkin};

const PERSONA: &str = r#"{
  "minecraft:geometry": [{
    "description": { "identifier": "geometry.persona_x", "texture_width": 256, "texture_height": 256 },
    "bones": [{ "name": "head", "cubes": [{ "origin": [-4, 24, -4], "size": [8, 8, 8], "uv": [88, 120] }] }]
  }]
}"#;
const RED: [u8; 4] = [200, 0, 0, 255];

fn offsets() -> PacketOffsets {
    FakeLayout::packet_offsets()
}

fn reader() -> GeometryReader {
    GeometryReader::new(FakeGeometry::to_styled_string_address(), FakeGeometry::heap())
}

// a 256 atlas whose classic head region is transparent and whose head is at (96,128)
fn persona_skin() -> FakeSkin {
    let mut skin = FakeSkin::rgba(256, 256, [0, 0, 0, 0]);
    skin.paint(96, 128, 8, RED);
    skin
}

fn read(reader: &GeometryReader, skin: &FakeSkin) -> alaydriem_heads::structs::GeometryRead {
    // SAFETY: the fake skin lives for the call and has the 26.60 layout; the fake function builds the string.
    unsafe { reader.read(skin.skin_ref(), &offsets(), false) }.unwrap()
}

#[test]
fn persona_json_gives_the_head_and_frees_the_string() {
    FakeGeometry::set_json(PERSONA);
    let skin = persona_skin();

    let result = read(&reader(), &skin);

    assert_eq!(result.head, Some(HeadUv::new(UvRect::new(96.0, 128.0, 8.0, 8.0), None, 256, 256)));
    assert_eq!(result.json.as_deref(), Some(PERSONA));
    assert_eq!(FakeGeometry::last_this(), skin.element() + 0xF8);
    assert_eq!(FakeGeometry::live_allocations(), 0);
}

#[test]
fn null_and_empty_json_give_no_head() {
    let skin = persona_skin();
    for text in ["null", ""] {
        FakeGeometry::set_json(text);

        let result = read(&reader(), &skin);

        assert_eq!(result.head, None);
        assert_eq!(result.json.as_deref(), Some(text));
        assert_eq!(FakeGeometry::live_allocations(), 0);
    }
}

#[test]
fn a_string_over_one_mebibyte_is_skipped_and_still_freed() {
    FakeGeometry::set_json(&" ".repeat(GeometryReader::MAX_JSON + 1));
    let skin = persona_skin();

    let result = read(&reader(), &skin);

    assert_eq!(result.head, None);
    assert_eq!(result.json, None);
    assert_eq!(FakeGeometry::live_allocations(), 0);
}

#[test]
fn a_null_skin_ref_reads_nothing() {
    // SAFETY: a null reference is rejected before any read.
    assert!(unsafe { reader().read(0, &offsets(), false) }.is_none());
}

#[test]
fn the_slowest_read_is_kept() {
    FakeGeometry::set_json(PERSONA);
    let reader = reader();
    assert_eq!(reader.slowest_nanos(), 0);

    read(&reader, &persona_skin());

    assert!(reader.slowest_nanos() > 0);
}

fn observer(geometry: bool, diagnostics: SkinDiagnostics) -> (PacketObserver, crossbeam_channel::Receiver<FaceUpdate>) {
    let (sender, receiver) = bounded(8);
    let observer = if geometry {
        PacketObserver::new_with_geometry(sender, offsets(), diagnostics, reader())
    } else {
        PacketObserver::new_with_diagnostics(sender, offsets(), diagnostics)
    };
    (observer, receiver)
}

fn one_player(skin: &FakeSkin) -> FakePacket {
    FakePacket::player_list(&[FakeEntry { variant: 1, uuid: [10; 16], actor: 5, skin: Some(skin) }])
}

#[test]
fn a_persona_skin_gets_its_face_from_the_geometry() {
    FakeGeometry::set_json(PERSONA);
    let skin = persona_skin();
    let packet = one_player(&skin);
    let (observer, receiver) = observer(true, SkinDiagnostics::new());

    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    match receiver.try_recv().unwrap() {
        FaceUpdate::Add { face, .. } => assert_eq!(face.pixel(0, 0), RED),
        other => panic!("unexpected update {other:?}"),
    }
    assert_eq!(FakeGeometry::live_allocations(), 0);
}

#[test]
fn without_a_usable_geometry_the_classic_head_is_used() {
    FakeGeometry::set_json("null");
    let classic = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let atlas = persona_skin();
    let (observer, receiver) = observer(true, SkinDiagnostics::new());

    let packet = one_player(&classic);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };
    assert!(matches!(receiver.try_recv().unwrap(), FaceUpdate::Add { .. }));

    // the persona atlas has nothing at the classic head, so no face is sent
    let packet = one_player(&atlas);
    // SAFETY: as above.
    unsafe { observer.on_player_list(packet.addr()) };
    assert!(receiver.try_recv().is_err());
}

const BLUE: [u8; 4] = [0, 0, 200, 255];
const GREEN: [u8; 4] = [0, 200, 0, 255];

// the persona atlas of `persona_skin` plus a 32x64 animated face: face at (8,8) and hat hair over its top row
fn persona_with_animated_face() -> FakeSkin {
    let mut skin = persona_skin();
    skin.mark(false, true);
    skin.set_animated_face(32, 64, &[(8, 8, 8, BLUE), (8, 24, 1, GREEN), (9, 24, 1, GREEN)]);
    skin
}

fn sent_face(
    observer: &PacketObserver,
    receiver: &crossbeam_channel::Receiver<FaceUpdate>,
    skin: &FakeSkin,
) -> Box<heads_core::structs::FaceImage> {
    let packet = one_player(skin);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };
    match receiver.try_recv().unwrap() {
        FaceUpdate::Add { face, .. } => face,
        other => panic!("unexpected update {other:?}"),
    }
}

#[test]
fn a_persona_face_comes_from_the_animated_face_image_with_the_hat_over_it() {
    FakeGeometry::set_json(PERSONA);
    let skin = persona_with_animated_face();
    let (observer, receiver) = observer(true, SkinDiagnostics::new());

    let face = sent_face(&observer, &receiver, &skin);

    assert_eq!(face.pixel(20, 20), BLUE);
    assert_eq!(face.pixel(2, 1), GREEN);
    assert_eq!(FakeGeometry::live_allocations(), 0);
}

#[test]
fn the_animated_face_mesh_names_where_the_face_is() {
    // the mesh puts the face at (0,0) and has no hat
    FakeGeometry::set_json(
        r#"{ "minecraft:geometry": [{
          "description": { "identifier": "geometry.animated_face_persona-x", "texture_width": 32, "texture_height": 64 },
          "bones": [{ "name": "head", "poly_mesh": {
            "normalized_uvs": true, "normals": [[0, 0, -1]], "positions": [[0, 0, 0]],
            "uvs": [[0.0, 1.0], [0.25, 1.0], [0.25, 0.875], [0.0, 0.875]],
            "polys": [[[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3]]]
          }}]
        }]}"#,
    );
    let mut skin = persona_with_animated_face();
    skin.set_animated_face(32, 64, &[(0, 0, 8, RED), (8, 8, 8, BLUE)]);
    let (observer, receiver) = observer(true, SkinDiagnostics::new());

    let face = sent_face(&observer, &receiver, &skin);

    assert_eq!(face.pixel(20, 20), RED);
}

#[test]
fn an_empty_animated_face_falls_back_to_the_geometry_head() {
    FakeGeometry::set_json(PERSONA);
    let mut skin = persona_with_animated_face();
    skin.set_animated_face(32, 64, &[]);
    let (observer, receiver) = observer(true, SkinDiagnostics::new());

    let face = sent_face(&observer, &receiver, &skin);

    assert_eq!(face.pixel(20, 20), RED);
}

#[test]
fn a_classic_skin_with_an_animated_face_and_no_mesh_keeps_the_classic_head() {
    FakeGeometry::set_json("null");
    let mut skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    skin.set_animated_face(32, 64, &[(8, 8, 8, BLUE)]);
    let (observer, receiver) = observer(true, SkinDiagnostics::new());

    let face = sent_face(&observer, &receiver, &skin);

    assert_eq!(face.pixel(20, 20), [10, 20, 30, 255]);
}

fn observer_with_skin_offsets(
    skin: alaydriem_heads::structs::SkinOffsets,
    diagnostics: SkinDiagnostics,
) -> (PacketObserver, crossbeam_channel::Receiver<FaceUpdate>) {
    let (sender, receiver) = bounded(8);
    let offsets = PacketOffsets { skin, ..offsets() };
    (PacketObserver::new_with_geometry(sender, offsets, diagnostics, reader()), receiver)
}

#[test]
fn without_the_geometry_name_the_head_is_not_located_and_the_classic_head_is_used() {
    FakeGeometry::set_json(PERSONA);
    let skin = persona_skin();
    let off = alaydriem_heads::structs::SkinOffsets { default_geometry_name: None, ..offsets().skin };
    let (observer, receiver) = observer_with_skin_offsets(off, SkinDiagnostics::new());

    let packet = one_player(&skin);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    // the atlas has nothing at the classic head, so no face is sent
    assert!(receiver.try_recv().is_err());
    assert_eq!(FakeGeometry::live_allocations(), 0);
}

#[test]
fn without_the_persona_offsets_the_animated_face_is_ignored_and_the_geometry_head_is_used() {
    FakeGeometry::set_json(PERSONA);
    let skin = persona_with_animated_face();
    let off = alaydriem_heads::structs::SkinOffsets { is_persona: None, animated: None, ..offsets().skin };
    let (observer, receiver) = observer_with_skin_offsets(off, SkinDiagnostics::new());

    let face = sent_face(&observer, &receiver, &skin);

    assert_eq!(face.pixel(20, 20), RED);
}

#[test]
fn without_the_diagnostics_offsets_the_faces_are_sent_and_nothing_is_logged() {
    FakeGeometry::set_json(PERSONA);
    let skin = persona_skin();
    let off = alaydriem_heads::structs::SkinOffsets { resource_patch: None, is_premium: None, ..offsets().skin };
    let (observer, receiver) = observer_with_skin_offsets(off, SkinDiagnostics::new());

    let face = sent_face(&observer, &receiver, &skin);

    assert_eq!(face.pixel(0, 0), RED);
    assert_eq!(observer.diagnostics().logged(), 0);
}
