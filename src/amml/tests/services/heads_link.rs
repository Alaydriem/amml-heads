use std::path::Path;

use alaydriem_heads::services::HeadsLink;
use amm_client::test_support::FakeTable;
use amm_core::structs::event::ModEvent;
use heads_core::config::HeadsSettings;

fn link() -> HeadsLink {
    HeadsLink::new(FakeTable::table(), Path::new(FakeTable::CONFIG_DIR)).unwrap()
}

#[test]
fn config_error_falls_back_to_defaults() {
    let link = link();
    FakeTable::set_config("heads {");

    assert_eq!(link.settings(), HeadsSettings::default());
}

#[test]
fn settings_are_read_and_clamped() {
    let link = link();
    FakeTable::set_config("heads {\n  enabled = false\n  scale = 9.0\n}\n");

    let settings = link.settings();

    assert!(!settings.enabled);
    assert_eq!(settings.scale(), 2.0);
}

#[test]
fn poll_reports_only_settings_changes() {
    let mut link = link();
    assert!(!link.poll());

    FakeTable::push_event(ModEvent::Enabled);
    assert!(!link.poll());

    FakeTable::push_event(ModEvent::Enabled);
    FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });
    assert!(link.poll());
}
