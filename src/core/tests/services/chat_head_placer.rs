use std::sync::Arc;

use heads_core::services::{CellFlags, ChatHeadPlacer, GlyphCells, PlayerDirectory};
use heads_core::structs::{ChatLine, MessageKind};

fn uuid(n: u8) -> [u8; 16] {
    [n; 16]
}

struct World {
    cells: GlyphCells,
    directory: PlayerDirectory,
}

impl World {
    fn new(players: &[(u8, &str, &str, bool)]) -> World {
        let mut cells = GlyphCells::new(Arc::new(CellFlags::new()));
        let mut directory = PlayerDirectory::new();
        for (n, xuid, name, face) in players {
            directory.record(uuid(*n), xuid, name);
            if let Some(cell) = cells.cell_for(uuid(*n))
                && *face
            {
                cells.mark_face(cell);
            }
        }
        World { cells, directory }
    }

    fn place(&self, kind: MessageKind, full: &str, username: &str, xuid: &str) -> Option<usize> {
        ChatHeadPlacer::place(&ChatLine { kind, full, username, author_xuid: xuid }, &self.cells, &self.directory)
    }

    fn glyph(&self, kind: MessageKind, full: &str, username: &str, xuid: &str) -> Option<char> {
        ChatHeadPlacer::glyph_for(&ChatLine { kind, full, username, author_xuid: xuid }, &self.cells, &self.directory)
    }

    fn edit(&self, kind: MessageKind, full: &str, username: &str, xuid: &str) -> Option<String> {
        ChatHeadPlacer::edit(&ChatLine { kind, full, username, author_xuid: xuid }, &self.cells, &self.directory)
    }
}

fn two_players() -> World {
    World::new(&[(1, "x1", "Steve", true), (2, "x2", "Alex", true)])
}

#[test]
fn a_chat_line_gets_the_glyph_at_the_start() {
    let world = two_players();
    assert_eq!(world.place(MessageKind::Chat, "<Steve> hi", "Steve", "x1"), Some(0));
    assert_eq!(world.glyph(MessageKind::Chat, "<Steve> hi", "Steve", "x1"), Some('\u{E110}'));
    assert_eq!(world.glyph(MessageKind::Chat, "<Alex> hi", "Alex", "x2"), Some('\u{E111}'));
}

#[test]
fn a_whisper_gets_the_glyph_at_the_start() {
    let world = two_players();
    assert_eq!(world.place(MessageKind::Whisper, "Steve whispers to you", "Steve", "x1"), Some(0));
}

#[test]
fn an_empty_xuid_falls_back_to_the_username() {
    let world = two_players();
    assert_eq!(world.glyph(MessageKind::Chat, "<Alex> hi", "Alex", ""), Some('\u{E111}'));
    assert_eq!(world.place(MessageKind::Chat, "<Nobody> hi", "Nobody", ""), None);
    assert_eq!(world.place(MessageKind::Chat, "<Alex> hi", "Alex", "unknown"), None);
}

#[test]
fn a_join_line_gets_the_glyph_before_the_name() {
    let world = two_players();
    assert_eq!(world.place(MessageKind::Localized, "Steve joined the game", "", ""), Some(0));
    assert_eq!(world.place(MessageKind::Localized, "* Steve waves", "", ""), Some(2));
}

#[test]
fn the_glyph_goes_after_the_formatting_codes_before_the_name() {
    let world = two_players();
    let full = "\u{a7}eSteve joined the game";
    let at = world.place(MessageKind::Localized, full, "", "").unwrap();
    assert_eq!(at, 3);
    assert_eq!(ChatHeadPlacer::insert(full, at, '\u{E110}'), "\u{a7}e\u{E110}Steve joined the game");
}

#[test]
fn a_name_inside_a_longer_name_is_not_matched() {
    let world = World::new(&[(1, "x1", "Steve", true), (2, "x2", "Steve2", true)]);
    assert_eq!(world.glyph(MessageKind::Localized, "Steve2 joined the game", "", ""), Some('\u{E111}'));
    assert_eq!(world.glyph(MessageKind::Localized, "Steve joined the game", "", ""), Some('\u{E110}'));
    let only_short = World::new(&[(1, "x1", "Steve", true)]);
    assert_eq!(only_short.place(MessageKind::Localized, "Steve2 joined the game", "", ""), None);
    assert_eq!(only_short.place(MessageKind::Localized, "a_Steve joined", "", ""), None);
}

#[test]
fn a_line_that_names_two_players_gets_no_head() {
    let world = two_players();
    assert_eq!(world.place(MessageKind::Localized, "Steve gave Alex a cookie", "", ""), None);
    assert_eq!(world.place(MessageKind::Localized, "Steve and Steve", "", ""), None);
}

#[test]
fn two_players_with_one_name_get_no_head_in_a_join_line() {
    let world = World::new(&[(1, "x1", "Alex", true), (2, "x2", "Alex", true)]);
    assert_eq!(world.place(MessageKind::Localized, "Alex joined the game", "", ""), None);
}

#[test]
fn a_name_that_is_not_ascii_is_found_on_a_character_boundary() {
    let world = World::new(&[(1, "x1", "Zo\u{eb}", true)]);
    let full = "\u{a7}eZo\u{eb} joined the game";
    let at = world.place(MessageKind::Localized, full, "", "").unwrap();
    assert!(full.is_char_boundary(at));
    let inserted = ChatHeadPlacer::insert(full, at, '\u{E110}');
    assert_eq!(inserted, "\u{a7}e\u{E110}Zo\u{eb} joined the game");
    assert_eq!(world.place(MessageKind::Localized, "Zo\u{eb}x joined", "", ""), None);
}

#[test]
fn a_player_without_a_face_gets_no_head() {
    let world = World::new(&[(1, "x1", "Steve", false)]);
    assert_eq!(world.place(MessageKind::Chat, "<Steve> hi", "Steve", "x1"), None);
    assert_eq!(world.glyph(MessageKind::Localized, "Steve joined", "", ""), None);
}

#[test]
fn other_message_kinds_get_no_head() {
    let world = two_players();
    let kind = MessageKind::from_raw(4);
    assert_eq!(kind, MessageKind::Other);
    assert_eq!(world.place(kind, "Steve joined the game", "Steve", "x1"), None);
}

#[test]
fn a_player_named_like_a_word_in_the_line_gets_no_head() {
    let world = World::new(&[(1, "x1", "joined", true)]);
    assert_eq!(world.place(MessageKind::Localized, "Alex joined the game", "", ""), None);
}

#[test]
fn an_emote_arrives_as_a_chat_line_without_a_username() {
    let world = two_players();
    assert_eq!(world.place(MessageKind::Chat, "* Steve waves", "", "x2"), Some(2));
    assert_eq!(world.glyph(MessageKind::Chat, "* Steve waves", "", ""), Some('\u{E110}'));
    assert_eq!(world.place(MessageKind::Chat, "* Nobody waves", "", ""), None);
}

#[test]
fn a_name_in_the_emote_body_gets_no_head() {
    let world = World::new(&[(1, "x1", "Bob", true)]);
    assert_eq!(world.place(MessageKind::Localized, "* Alex waves to Bob", "", ""), None);
}

#[test]
fn two_names_that_both_fit_the_start_of_the_line_give_no_head() {
    let world = World::new(&[(1, "x1", "Steve", true), (2, "x2", "Steve Jr", true)]);
    assert_eq!(world.place(MessageKind::Localized, "Steve Jr joined the game", "", ""), None);
}

#[test]
fn a_face_code_point_typed_into_a_line_is_removed() {
    let world = two_players();
    let full = "<Alex> look \u{E110} at \u{E1FF}me\u{E111}";
    assert_eq!(world.edit(MessageKind::Chat, full, "Alex", "x2").as_deref(), Some("\u{E111}<Alex> look  at me"));
    let join = "\u{a7}eSteve joined the game\u{E111}";
    assert_eq!(
        world.edit(MessageKind::Localized, join, "", "").as_deref(),
        Some("\u{a7}e\u{E110}Steve joined the game")
    );
}

#[test]
fn a_line_without_a_head_is_cleaned_of_face_code_points() {
    let world = two_players();
    let full = "<Nobody> hi \u{E111}there";
    assert_eq!(world.edit(MessageKind::Chat, full, "Nobody", "").as_deref(), Some("<Nobody> hi there"));
    assert_eq!(world.edit(MessageKind::Other, "\u{E111}", "", "").as_deref(), Some(""));
}

#[test]
fn a_line_without_a_head_or_a_face_code_point_is_not_edited() {
    let world = two_players();
    assert_eq!(world.edit(MessageKind::Chat, "<Nobody> hi \u{E10F}\u{E200}", "Nobody", ""), None);
    assert_eq!(ChatHeadPlacer::clean("plain \u{E10F} text"), None);
}

#[test]
fn a_stale_name_and_a_new_unrecorded_player_with_that_name_give_no_head() {
    let mut world = World::new(&[(1, "x1", "Alex", true)]);
    world.directory.note(uuid(9), "", "Alex");
    assert_eq!(world.place(MessageKind::Chat, "<Alex> hi", "Alex", ""), None);
    assert_eq!(world.edit(MessageKind::Chat, "<Alex> hi", "Alex", ""), None);
    assert_eq!(world.place(MessageKind::Localized, "\u{a7}eAlex joined the game", "", ""), None);
    assert_eq!(world.place(MessageKind::Localized, "* Alex waves", "", ""), None);
}

#[test]
fn a_xuid_shared_with_an_unrecorded_player_gives_no_head() {
    let mut world = World::new(&[(1, "x1", "Alex", true)]);
    assert_eq!(world.glyph(MessageKind::Chat, "<Alex> hi", "Alex", "x1"), Some('\u{E110}'));
    world.directory.note(uuid(9), "x1", "Someone");
    assert_eq!(world.place(MessageKind::Chat, "<Alex> hi", "Alex", "x1"), None);
}
