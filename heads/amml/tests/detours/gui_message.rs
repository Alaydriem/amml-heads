use std::ffi::c_void;
use std::sync::atomic::Ordering;

use alaydriem_heads::detours::gui_message;
use alaydriem_heads::services::{ChatRuntime, ChatState};
use heads_core::services::GlyphPage;

use crate::support::{FakeFont, FakeGuiMessage, FakeHeap, FakeRig};

#[allow(clippy::too_many_arguments)]
extern "C" fn fake_original(
    this: usize,
    _kind: i32,
    _username: usize,
    _message: usize,
    _filtered: usize,
    _tts: usize,
    _xuid: usize,
    _lifetime: f32,
    _force_visible: u8,
    _tts_required: u8,
) -> usize {
    this
}

fn chat_with_alex(font_vtable: usize) -> FakeFont {
    gui_message::GUARD.original().store(fake_original as *const () as *mut c_void, Ordering::Release);
    ChatRuntime::install(ChatState::new(GlyphPage::new(&[]), ChatRuntime::flags()), ChatRuntime::flags());
    ChatRuntime::with_state_blocking(|state| {
        state.on_face([1; 16], "2535400000000001", "Alex", &FakeRig::face([1, 1, 1, 255]))
    });
    ChatRuntime::flush_page_with(|_| true);
    ChatRuntime::set_game_access(FakeFont::check(), FakeHeap::heap());
    let font = FakeFont::new(font_vtable);
    // SAFETY: the fake client lives for the call.
    unsafe { ChatRuntime::refresh_font(font.addr()) };
    font
}

fn run(message: &FakeGuiMessage, kind: i32) -> usize {
    gui_message::detour(message.addr(), kind, 0, 0, 0, 0, 0, 10.0, 0, 0)
}

#[test]
fn a_chat_line_gets_the_glyph_written_into_both_strings() {
    let _font = chat_with_alex(FakeFont::BITMAP_VTABLE);
    let message = FakeGuiMessage::new(
        1,
        "Alex",
        "<Alex> hello everyone on the server",
        Some("<Alex> hello ********"),
        "2535400000000001",
    );

    assert_eq!(run(&message, 1), message.addr());

    assert_eq!(message.full(), "\u{E110}<Alex> hello everyone on the server");
    assert_eq!(message.filtered().as_deref(), Some("\u{E110}<Alex> hello ********"));
}

#[test]
fn a_line_without_a_head_is_left_byte_for_byte() {
    let _font = chat_with_alex(FakeFont::BITMAP_VTABLE);
    let message = FakeGuiMessage::new(1, "Steve", "<Steve> hello everyone", Some("<Steve> h*"), "2535400000000009");
    let before = message.bytes();

    run(&message, 1);

    assert_eq!(message.bytes(), before);
}

#[test]
fn a_missing_filtered_string_stays_missing() {
    let _font = chat_with_alex(FakeFont::BITMAP_VTABLE);
    let message = FakeGuiMessage::new(3, "", "§eAlex joined the game", None, "");

    run(&message, 3);

    assert_eq!(message.full(), "§e\u{E110}Alex joined the game");
    assert_eq!(message.filtered(), None);
}

#[test]
fn a_font_that_is_not_a_bitmap_font_leaves_the_line_alone() {
    let _font = chat_with_alex(FakeFont::SMOOTH_VTABLE);
    let message = FakeGuiMessage::new(1, "Alex", "<Alex> hello everyone", None, "2535400000000001");
    let before = message.bytes();

    run(&message, 1);

    assert_eq!(message.bytes(), before);
}

#[test]
fn an_edited_line_frees_each_old_heap_string_once_with_its_size() {
    let _font = chat_with_alex(FakeFont::BITMAP_VTABLE);
    let full = "<Alex> hello everyone on the server";
    let filtered = "<Alex> hello ********";
    let message = FakeGuiMessage::new(1, "Alex", full, Some(filtered), "2535400000000001");
    let allocated = FakeHeap::allocated();
    let freed = FakeHeap::freed();
    let sizes = FakeHeap::freed_sizes().len();

    run(&message, 1);

    assert_eq!(FakeHeap::allocated() - allocated, 2);
    assert_eq!(FakeHeap::freed() - freed, 2);
    // a string built from `text` has capacity `text.len()`, and its block is `capacity + 1` bytes
    assert_eq!(FakeHeap::freed_sizes()[sizes..], [full.len() + 1, filtered.len() + 1]);
}

#[test]
fn a_typed_face_code_point_is_removed_from_a_line_without_a_head() {
    let _font = chat_with_alex(FakeFont::BITMAP_VTABLE);
    let message =
        FakeGuiMessage::new(1, "Steve", "<Steve> look \u{E110}", Some("<Steve> \u{E111}*"), "2535400000000009");

    run(&message, 1);

    assert_eq!(message.full(), "<Steve> look ");
    assert_eq!(message.filtered().as_deref(), Some("<Steve> *"));
}
