use alaydriem_heads::services::ChatBoard;
use amml_api::testing::FakeGame;
use heads_core::services::PlayerDirectory;
use heads_core::structs::{ChatLine, IconImage, MessageKind};

use crate::support::heads_rig::HeadsRig;

fn uuid(n: usize) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&(n as u64).to_le_bytes());
    bytes
}

fn image() -> Box<IconImage> {
    Box::new(IconImage::new([200; 1024]))
}

fn game() -> FakeGame {
    let game = HeadsRig::served_game();
    game.open_gate();
    game
}

fn steve_says(board: &ChatBoard, show: bool) -> Option<String> {
    let line = ChatLine { kind: MessageKind::Chat, full: "<Steve> hi", username: "Steve", author_xuid: "x1" };
    board.glyph_line(&line, show)
}

#[test]
fn a_player_gets_an_icon_and_its_glyph_goes_into_the_line() {
    let game = game();
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());

    let made = board.register_waiting(&game.ctx().chat(), 16);

    assert_eq!(made, vec![('\u{E110}', true)]);
    assert_eq!(board.glyph_of(uuid(1)), Some('\u{E110}'));
    assert_eq!(steve_says(&board, true).as_deref(), Some("\u{E110}<Steve> hi"));
}

#[test]
fn a_line_stays_as_it_is_when_icons_are_not_shown() {
    let game = game();
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());
    board.register_waiting(&game.ctx().chat(), 16);

    assert_eq!(steve_says(&board, false), None);
}

#[test]
fn a_face_character_a_server_wrote_is_removed_when_icons_are_not_shown() {
    let board = ChatBoard::new();
    let line = ChatLine { kind: MessageKind::Chat, full: "<Eve> \u{E115}hi", username: "Eve", author_xuid: "" };

    assert_eq!(board.glyph_line(&line, false).as_deref(), Some("<Eve> hi"));
}

#[test]
fn the_waiting_images_stop_at_240_players() {
    let mut board = ChatBoard::new();
    for n in 0..PlayerDirectory::CAPACITY {
        assert!(board.on_face(uuid(n), &format!("x{n}"), &format!("Player{n}"), image()));
    }

    assert!(!board.on_face(uuid(240), "x240", "Late", image()));
    assert_eq!(board.waiting(), PlayerDirectory::CAPACITY);
}

#[test]
fn a_second_image_of_a_player_replaces_the_waiting_one() {
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());

    board.on_skin(uuid(1), image());

    assert_eq!(board.waiting(), 1);
}

#[test]
fn a_skin_of_a_player_without_an_icon_is_ignored() {
    let mut board = ChatBoard::new();
    board.on_identity(uuid(1), "x1", "Steve");

    board.on_skin(uuid(1), image());
    board.on_skin(uuid(2), image());

    assert_eq!(board.waiting(), 0);
}

#[test]
fn an_icon_is_updated_in_place_for_a_new_skin() {
    let game = game();
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());
    board.register_waiting(&game.ctx().chat(), 16);
    board.on_skin(uuid(1), image());

    let made = board.register_waiting(&game.ctx().chat(), 16);

    assert_eq!(made, vec![('\u{E110}', false)]);
    assert_eq!(board.icons(), 1);
    assert_eq!(game.icons().len(), 1);
}

#[test]
fn an_icon_of_a_left_world_is_registered_again() {
    let game = game();
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());
    board.register_waiting(&game.ctx().chat(), 16);
    game.leave_icon_world();
    board.on_skin(uuid(1), image());

    let made = board.register_waiting(&game.ctx().chat(), 16);

    assert_eq!(made.len(), 1);
    assert!(made[0].1);
    assert_eq!(board.icons(), 1);
}

#[test]
fn images_wait_while_the_font_is_not_a_bitmap_font() {
    let game = game();
    game.set_bitmap_font(false);
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());
    board.on_face(uuid(2), "x2", "Alex", image());

    let made = board.register_waiting(&game.ctx().chat(), 16);

    assert!(made.is_empty());
    assert_eq!(board.waiting(), 2);
    game.set_bitmap_font(true);
    assert_eq!(board.register_waiting(&game.ctx().chat(), 16).len(), 2);
    assert_eq!(board.waiting(), 0);
}

#[test]
fn only_the_limit_is_registered_in_one_call() {
    let game = game();
    let mut board = ChatBoard::new();
    for n in 0..5 {
        board.on_face(uuid(n), &format!("x{n}"), &format!("Player{n}"), image());
    }

    assert_eq!(board.register_waiting(&game.ctx().chat(), 2).len(), 2);
    assert_eq!(board.waiting(), 3);
}

#[test]
fn clear_forgets_the_players_and_frees_the_icons() {
    let game = game();
    let mut board = ChatBoard::new();
    board.on_face(uuid(1), "x1", "Steve", image());
    board.on_face(uuid(2), "x2", "Alex", image());
    board.register_waiting(&game.ctx().chat(), 1);

    board.clear();

    assert_eq!((board.waiting(), board.icons()), (0, 0));
    assert!(game.icons().is_empty());
    assert_eq!(steve_says(&board, true), None);
}
