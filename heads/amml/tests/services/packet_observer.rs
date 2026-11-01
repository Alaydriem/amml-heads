use alaydriem_heads::services::{ChatRuntime, ChatState, PacketObserver};
use alaydriem_heads::structs::PacketOffsets;
use crossbeam_channel::{Receiver, bounded};
use heads_core::services::GlyphPage;
use heads_core::structs::{ChatLine, FaceImage, FaceUpdate, MessageKind};

use crate::support::{FakeEntry, FakePacket, FakeSkin};

fn observer(capacity: usize) -> (PacketObserver, Receiver<FaceUpdate>) {
    let (sender, receiver) = bounded(capacity);
    (PacketObserver::new(sender, PacketOffsets::new(0xD8, 0xD0, 0xA8, 0x80, 0xF8)), receiver)
}

fn add<'a>(n: u8, actor: i64, skin: &'a FakeSkin) -> FakeEntry<'a> {
    FakeEntry { variant: 1, uuid: [n; 16], actor, skin: Some(skin) }
}

#[test]
fn player_list_add_sends_add_update() {
    let skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let packet = FakePacket::player_list(&[add(5, 77, &skin)]);
    let (observer, receiver) = observer(8);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    match receiver.try_recv().unwrap() {
        FaceUpdate::Add { uuid, actor, face } => {
            assert_eq!(uuid, [5; 16]);
            assert_eq!(actor, 77);
            assert_eq!(face.pixel(0, 0), [10, 20, 30, 255]);
        }
        other => panic!("unexpected update {other:?}"),
    }
    assert!(receiver.try_recv().is_err());
}

#[test]
fn player_list_remove_sends_remove() {
    let packet = FakePacket::player_list(&[FakeEntry { variant: 0, uuid: [9; 16], actor: 0, skin: None }]);
    let (observer, receiver) = observer(8);
    // SAFETY: the fake packet lives for the call and matches the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert_eq!(receiver.try_recv().unwrap(), FaceUpdate::Remove { uuid: [9; 16] });
}

#[test]
fn unknown_variant_index_is_skipped() {
    let skin = FakeSkin::rgba(64, 64, [1, 2, 3, 255]);
    let packet = FakePacket::player_list(&[
        FakeEntry { variant: 7, uuid: [1; 16], actor: 1, skin: Some(&skin) },
        add(2, 2, &skin),
    ]);
    let (observer, receiver) = observer(8);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    let updates: Vec<FaceUpdate> = receiver.try_iter().collect();
    assert_eq!(updates.len(), 1);
    assert!(matches!(updates[0], FaceUpdate::Add { actor: 2, .. }));
}

#[test]
fn bad_skin_in_add_sends_nothing() {
    let rgb = FakeSkin::new(64, 64, 3, 64 * 64 * 4, [1, 2, 3, 255]);
    let odd = FakeSkin::rgba(96, 96, [1, 2, 3, 255]);
    let packet = FakePacket::player_list(&[add(1, 1, &rgb), add(2, 2, &odd)]);
    let (observer, receiver) = observer(8);
    // SAFETY: the fake packet and skins live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert!(receiver.try_recv().is_err());
}

#[test]
fn null_packet_sends_nothing() {
    let (observer, receiver) = observer(8);
    // SAFETY: a null packet is rejected before any read.
    unsafe {
        observer.on_player_list(0);
        observer.on_player_skin(0);
    }
    assert!(receiver.try_recv().is_err());
}

#[test]
fn player_skin_sends_skin_update() {
    let skin = FakeSkin::rgba(64, 64, [40, 50, 60, 255]);
    let packet = FakePacket::player_skin([3; 16], &skin);
    let (observer, receiver) = observer(8);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_skin(packet.addr()) };

    match receiver.try_recv().unwrap() {
        FaceUpdate::Skin { uuid, face } => {
            assert_eq!(uuid, [3; 16]);
            assert_eq!(face.pixel(31, 31), [40, 50, 60, 255]);
        }
        other => panic!("unexpected update {other:?}"),
    }
}

#[test]
fn full_channel_drops_without_blocking() {
    let skin = FakeSkin::rgba(64, 64, [1, 2, 3, 255]);
    let packet = FakePacket::player_list(&[add(1, 1, &skin), add(2, 2, &skin), add(3, 3, &skin)]);
    let (observer, receiver) = observer(1);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert_eq!(receiver.len(), 1);
    assert_eq!(observer.dropped(), 2);
}

#[test]
fn full_channel_skips_extracting_the_skin() {
    let good = FakeSkin::rgba(64, 64, [1, 2, 3, 255]);
    let bad = FakeSkin::new(64, 64, 3, 64 * 64 * 4, [1, 2, 3, 255]);
    let packet = FakePacket::player_list(&[add(1, 1, &good), add(2, 2, &bad)]);
    let (observer, receiver) = observer(1);
    // SAFETY: the fake packet and skins live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert_eq!(receiver.len(), 1);
    // the second entry is dropped before its bad skin is read, so it counts as a drop
    assert_eq!(observer.dropped(), 1);
}

fn chat_observer() -> (PacketObserver, Receiver<FaceUpdate>) {
    let (sender, receiver) = bounded(8);
    let offsets = PacketOffsets::new_with_chat(0xD8, 0xD0, 0xA8, 0x80, 0xF8, 0x20, 0x40);
    (PacketObserver::new(sender, offsets), receiver)
}

fn install_chat_state() {
    ChatRuntime::install(ChatState::new(GlyphPage::new(&[]), ChatRuntime::flags()), ChatRuntime::flags());
    ChatRuntime::flush_page_with(|_| true);
}

fn glyph_for_steve() -> Option<String> {
    let line = ChatLine { kind: MessageKind::Chat, full: "hi", username: "Steve", author_xuid: "2535" };
    ChatRuntime::with_state(|state| state.glyph_line(&line)).flatten()
}

#[test]
fn player_list_add_feeds_the_chat_state_with_name_and_xuid() {
    install_chat_state();
    let skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let packet = FakePacket::player_list_named(&[add(5, 77, &skin)], &[("Steve", "2535")]);
    let (observer, receiver) = chat_observer();
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert_eq!(glyph_for_steve().as_deref(), Some("\u{E110}hi"));
    assert!(ChatRuntime::has_face(0xE110));
    assert!(matches!(receiver.try_recv().unwrap(), FaceUpdate::Add { actor: 77, .. }));
}

#[test]
fn a_name_longer_than_the_bound_is_not_fed_to_the_chat_state() {
    install_chat_state();
    let skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let long = "n".repeat(65);
    let packet = FakePacket::player_list_named(&[add(5, 77, &skin)], &[(long.as_str(), "2535")]);
    let (observer, receiver) = chat_observer();
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert_eq!(glyph_for_steve(), None);
    assert!(!ChatRuntime::has_face(0xE110));
    assert!(matches!(receiver.try_recv().unwrap(), FaceUpdate::Add { .. }));
}

#[test]
fn without_chat_offsets_or_state_the_face_still_reaches_the_channel() {
    let skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let packet = FakePacket::player_list_named(&[add(5, 77, &skin)], &[("Steve", "2535")]);
    let (observer, receiver) = observer(8);
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert!(matches!(receiver.try_recv().unwrap(), FaceUpdate::Add { actor: 77, .. }));
    assert_eq!(glyph_for_steve(), None);
}

#[test]
fn player_skin_redraws_the_cell_of_a_known_player() {
    install_chat_state();
    ChatRuntime::with_state(|state| {
        state.on_face([3; 16], "2535", "Steve", &FaceImage::new([0; 4096]));
        state.take_page();
    });
    let skin = FakeSkin::rgba(64, 64, [40, 50, 60, 255]);
    let packet = FakePacket::player_skin([3; 16], &skin);
    let (observer, _receiver) = chat_observer();
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_skin(packet.addr()) };

    let first =
        ChatRuntime::with_state(|state| state.take_page().map(|page| page[18 * 256 * 4..18 * 256 * 4 + 4].to_vec()));
    assert_eq!(first.flatten(), Some(vec![40, 50, 60, 255]));
}

#[test]
fn an_add_whose_skin_cannot_be_read_still_makes_a_shared_identity_give_no_head() {
    install_chat_state();
    let good = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let first = FakePacket::player_list_named(&[add(5, 77, &good)], &[("Steve", "2535")]);
    let (observer, _receiver) = chat_observer();
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(first.addr()) };
    assert_eq!(glyph_for_steve().as_deref(), Some("\u{E110}hi"));

    let bad = FakeSkin::new(64, 64, 3, 64 * 64 * 4, [1, 2, 3, 255]);
    let second = FakePacket::player_list_named(&[add(6, 78, &bad)], &[("Steve", "2535")]);
    // SAFETY: as above.
    unsafe { observer.on_player_list(second.addr()) };

    assert_eq!(glyph_for_steve(), None);
    assert!(!ChatRuntime::has_face(0xE111));
}

#[test]
fn an_add_dropped_by_a_full_channel_still_makes_a_shared_name_give_no_head() {
    install_chat_state();
    ChatRuntime::with_state(|state| state.on_face([3; 16], "", "Steve", &FaceImage::new([0; 4096])));
    let skin = FakeSkin::rgba(64, 64, [10, 20, 30, 255]);
    let packet = FakePacket::player_list_named(&[add(4, 1, &skin), add(6, 2, &skin)], &[("x", "1"), ("Steve", "")]);
    let (sender, receiver) = bounded(1);
    let observer = PacketObserver::new(sender, PacketOffsets::new_with_chat(0xD8, 0xD0, 0xA8, 0x80, 0xF8, 0x20, 0x40));
    // SAFETY: the fake packet and skin live for the call and match the 26.60 layout.
    unsafe { observer.on_player_list(packet.addr()) };

    assert_eq!(observer.dropped(), 1);
    assert_eq!(receiver.len(), 1);
    let line = ChatLine { kind: MessageKind::Chat, full: "hi", username: "Steve", author_xuid: "" };
    assert_eq!(ChatRuntime::with_state(|state| state.glyph_line(&line)).flatten(), None);
}
