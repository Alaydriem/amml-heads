use alaydriem_heads::services::{HeadsDescriptor, HeadsLink};
use amm_client::link::LinkState;
use amm_client::test_support::FakeTable;
use amm_core::structs::event::ModEvent;
use amml_api::testing::FakeGame;

use crate::support::heads_rig::HeadsRig;

fn link(config: &str) -> (FakeGame, HeadsLink) {
    let game = HeadsRig::served_game();
    FakeTable::provide(&game.ctx()).unwrap();
    FakeTable::set_config(config);
    let link = HeadsLink::start(&FakeTable::ctx_of(&game, HeadsDescriptor::TEXT)).unwrap();
    (game, link)
}

#[test]
fn the_settings_are_the_heads_block_of_the_config() {
    let (_game, link) = link("heads {\n  enabled = false\n  scale = 1.5\n  chat = false\n}\n");

    let settings = link.settings();

    assert!(!settings.enabled);
    assert_eq!(settings.scale(), 1.5);
    assert!(!settings.chat);
}

#[test]
fn a_config_that_does_not_read_gives_the_defaults() {
    let (_game, link) = link("heads {\n  scale = \"large\"\n}\n");

    let settings = link.settings();

    assert!(settings.enabled && settings.chat);
    assert_eq!(settings.scale(), 1.0);
}

#[test]
fn a_link_with_the_manager_present_is_connected() {
    let (_game, link) = link("");

    assert_eq!(link.state(), LinkState::Connected);
}

#[test]
fn a_poll_tells_that_the_settings_changed_and_ignores_other_events() {
    let (_game, mut link) = link("");
    FakeTable::push_event(ModEvent::Enabled);
    assert!(!link.poll());

    FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });

    assert!(link.poll());
    assert!(!link.poll());
}
