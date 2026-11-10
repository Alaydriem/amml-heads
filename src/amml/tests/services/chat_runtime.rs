use alaydriem_heads::services::{ChatRuntime, ChatState};
use heads_core::services::GlyphPage;
use heads_core::structs::{ChatLine, MessageKind};

use crate::support::FakeRig;

fn install() {
    ChatRuntime::install(ChatState::new(GlyphPage::new(&[]), ChatRuntime::flags()), ChatRuntime::flags());
    ChatRuntime::flush_page_with(|_| true);
}

fn glyph_line(username: &str, xuid: &str) -> Option<String> {
    let line = ChatLine { kind: MessageKind::Chat, full: "hi", username, author_xuid: xuid };
    ChatRuntime::with_state(|state| state.glyph_line(&line)).flatten()
}

#[test]
fn a_re_enable_keeps_every_cell() {
    install();
    ChatRuntime::with_state_blocking(|state| state.on_face([1; 16], "1", "Alex", &FakeRig::face([1, 1, 1, 255])));

    ChatRuntime::suspend();
    assert_eq!(glyph_line("Alex", "1"), None);
    ChatRuntime::with_state_blocking(|state| state.on_face([9; 16], "9", "Ghost", &FakeRig::face([1, 1, 1, 255])));
    install();
    ChatRuntime::with_state_blocking(|state| state.on_face([2; 16], "2", "Bob", &FakeRig::face([2, 2, 2, 255])));

    assert_eq!(glyph_line("Alex", "1").as_deref(), Some("\u{E110}hi"));
    assert_eq!(glyph_line("Bob", "2").as_deref(), Some("\u{E111}hi"));
    assert_eq!(glyph_line("Ghost", "9"), None);
}

#[test]
fn a_re_enable_offers_the_page_for_upload_again() {
    install();
    ChatRuntime::with_state(|state| state.flush_page(|_| true));
    ChatRuntime::suspend();
    ChatRuntime::with_state(|state| state.flush_page(|_| panic!("a suspended state uploads nothing")));

    ChatRuntime::install(ChatState::new(GlyphPage::new(&[]), ChatRuntime::flags()), ChatRuntime::flags());

    let mut offered = false;
    ChatRuntime::with_state(|state| {
        state.flush_page(|_| {
            offered = true;
            true
        })
    });
    assert!(offered);
}

#[test]
fn leaving_the_world_offers_the_page_again_on_the_next_flush() {
    install();
    ChatRuntime::flush_page_with(|_| panic!("the page did not change"));

    ChatRuntime::on_world_exit();

    let mut offered = 0;
    ChatRuntime::flush_page_with(|_| {
        offered += 1;
        true
    });
    ChatRuntime::flush_page_with(|_| {
        offered += 1;
        true
    });
    assert_eq!(offered, 1);
}
