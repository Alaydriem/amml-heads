use std::sync::Arc;

use alaydriem_heads::services::ChatState;
use heads_core::services::{CellFlags, GlyphPage};
use heads_core::structs::{ChatLine, MessageKind};

use crate::support::FakeRig;

const CELL_0_FIRST_PIXEL: usize = (16 + 2) * 256 * 4;

fn state() -> (ChatState, Arc<CellFlags>) {
    let flags = Arc::new(CellFlags::new());
    (ChatState::new(GlyphPage::new(&[]), Arc::clone(&flags)), flags)
}

fn line(full: &str) -> ChatLine<'_> {
    ChatLine { kind: MessageKind::Chat, full, username: "Steve", author_xuid: "2535" }
}

#[test]
fn a_face_gets_a_cell_and_dirties_the_page_once() {
    let (mut state, flags) = state();
    state.take_page();

    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));

    let page = state.take_page().expect("the page changed");
    assert_eq!(page[CELL_0_FIRST_PIXEL..CELL_0_FIRST_PIXEL + 4], [9, 8, 7, 255]);
    assert!(state.take_page().is_none());
    assert!(flags.has_face(0xE110));
    assert!(!flags.has_face(0xE111));
}

#[test]
fn a_skin_change_rewrites_the_same_cell() {
    let (mut state, flags) = state();
    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));
    state.take_page();

    state.on_skin([1; 16], &FakeRig::face([1, 2, 3, 255]));

    let page = state.take_page().expect("the page changed");
    assert_eq!(page[CELL_0_FIRST_PIXEL..CELL_0_FIRST_PIXEL + 4], [1, 2, 3, 255]);
    assert!(!flags.has_face(0xE111));
}

#[test]
fn a_skin_for_a_player_without_a_cell_does_nothing() {
    let (mut state, flags) = state();
    state.take_page();

    state.on_skin([7; 16], &FakeRig::face([1, 2, 3, 255]));

    assert!(state.take_page().is_none());
    assert!(!flags.has_face(0xE110));
}

#[test]
fn a_line_gets_no_glyph_while_the_setting_is_off() {
    let (mut state, _) = state();
    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));
    state.flush_page(|_| true);

    assert_eq!(state.glyph_line(&line("hello")).as_deref(), Some("\u{E110}hello"));
    state.set_chat(false);
    assert_eq!(state.glyph_line(&line("hello")), None);
    state.set_chat(true);
    assert_eq!(state.glyph_line(&line("hello")).as_deref(), Some("\u{E110}hello"));
}

#[test]
fn a_page_that_did_not_upload_is_offered_again() {
    let (mut state, _) = state();
    let mut offered = 0;

    state.flush_page(|page| {
        offered += 1;
        assert_eq!(page.len(), 256 * 256 * 4);
        false
    });
    state.flush_page(|_| {
        offered += 1;
        true
    });
    state.flush_page(|_| {
        offered += 1;
        true
    });

    assert_eq!(offered, 2);
}

#[test]
fn a_typed_face_code_point_is_removed_while_the_setting_is_off() {
    let (mut state, _) = state();
    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));
    state.flush_page(|_| true);
    state.set_chat(false);

    assert_eq!(state.glyph_line(&line("look \u{E110}here")).as_deref(), Some("look here"));
    assert_eq!(state.glyph_line(&line("hello")), None);

    state.suspend();
    assert_eq!(state.glyph_line(&line("look \u{E110}here")), None);
}

#[test]
fn no_glyph_is_put_in_before_the_page_has_uploaded() {
    let (mut state, _) = state();
    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));
    assert_eq!(state.glyph_line(&line("hello")), None);

    state.flush_page(|_| false);
    assert_eq!(state.glyph_line(&line("hello")), None);

    state.flush_page(|_| true);
    assert_eq!(state.glyph_line(&line("hello")).as_deref(), Some("\u{E110}hello"));
}

#[test]
fn a_face_code_point_is_removed_before_the_page_has_uploaded() {
    let (mut state, _) = state();
    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));

    assert_eq!(state.glyph_line(&line("hi \u{E111}")).as_deref(), Some("hi "));
}

#[test]
fn a_failing_upload_backs_off_after_three_failures() {
    let (mut state, _) = state();
    let mut offered = 0;

    for _ in 0..3 + 63 {
        state.flush_page(|_| {
            offered += 1;
            false
        });
    }
    assert_eq!(offered, 3);

    state.flush_page(|_| {
        offered += 1;
        true
    });
    assert_eq!(offered, 4);

    state.on_face([1; 16], "2535", "Steve", &FakeRig::face([9, 8, 7, 255]));
    state.flush_page(|_| {
        offered += 1;
        false
    });
    state.flush_page(|_| {
        offered += 1;
        true
    });
    assert_eq!(offered, 6);
}

#[test]
fn leaving_the_world_offers_the_page_again_at_once() {
    let (mut state, _) = state();
    for _ in 0..3 {
        state.flush_page(|_| false);
    }
    state.flush_page(|_| panic!("the back-off holds the next try"));

    state.on_world_exit();

    let mut offered = 0;
    state.flush_page(|_| {
        offered += 1;
        true
    });
    state.flush_page(|_| {
        offered += 1;
        true
    });
    state.on_world_exit();
    state.flush_page(|_| {
        offered += 1;
        true
    });
    assert_eq!(offered, 2);
}

#[test]
fn a_player_seen_without_a_face_makes_a_shared_name_give_no_head() {
    let (mut state, _) = state();
    state.on_face([1; 16], "", "Steve", &FakeRig::face([9, 8, 7, 255]));
    state.flush_page(|_| true);
    let by_name = ChatLine { kind: MessageKind::Chat, full: "hello", username: "Steve", author_xuid: "" };
    assert_eq!(state.glyph_line(&by_name).as_deref(), Some("\u{E110}hello"));

    state.on_identity([2; 16], "", "Steve");

    assert_eq!(state.glyph_line(&by_name), None);
}
