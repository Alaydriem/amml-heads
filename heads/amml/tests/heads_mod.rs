use std::path::Path;

use alaydriem_heads::HeadsMod;
use alaydriem_heads::entries::HeadsEntries;
use alaydriem_heads::services::{HeadsRuntime, RenderRuntime};
use alaydriem_heads::structs::TurnedOff;
use alaydriem_heads::types::LinkState;
use amm_client::test_support::FakeTable;
use amm_core::errors::LinkError;
use amm_core::structs::event::ModEvent;
use amml_game::EntryId;
use amml_sdk::{AmmlMod, SdkError};

use crate::support::{ErrorLine, FakeHost, FakeLog};

fn runtime() -> HeadsRuntime {
    HeadsRuntime::new(Path::new(FakeTable::CONFIG_DIR))
}

const LOCATOR_BAR_HOOKS: [&str; 6] = [
    "HudLocatorBarRenderer.drawIcon",
    "ClientNetworkHandler.handle.PlayerListPacket",
    "ClientNetworkHandler.handle.PlayerSkinPacket",
    "ClientExitLevel.hook",
    "ClientInstance.update",
    "HudLocatorBarRenderer.render",
];

#[test]
fn installs_the_locator_bar_hooks_then_the_chat_hooks_on_2660() {
    let host = FakeHost::for_2660();
    let runtime = runtime();

    HeadsMod::enable_with(&host, &runtime).unwrap();

    let mut expected = LOCATOR_BAR_HOOKS.to_vec();
    expected.extend([HeadsEntries::GUI_MESSAGE_CTOR.as_str(), HeadsEntries::BITMAP_FONT_GET_CHAR_WIDTH.as_str()]);
    assert_eq!(host.hooked(), expected);
    assert!(host.warnings().is_empty());
    assert!(host.errors().is_empty());
    assert_eq!(host.infos(), ["features off: none"]);
    assert!(RenderRuntime::is_installed());
    assert_eq!(runtime.link_state(), LinkState::Waiting);
}

#[test]
fn a_missing_chat_entry_turns_off_only_chat_heads() {
    for id in HeadsEntries::CHAT {
        let host = FakeHost::for_2660().without_entry(id);
        let runtime = runtime();

        HeadsMod::enable_with(&host, &runtime).unwrap();

        assert_eq!(host.hooked(), LOCATOR_BAR_HOOKS, "{}", id.as_str());
        assert!(host.infos().is_empty());
        let errors = host.errors();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].starts_with("features off: chat heads"), "{}", errors[0]);
        assert!(errors[0].contains(id.as_str()), "{}", errors[0]);
        assert_eq!(runtime.turned_off().len(), 1);
        assert_eq!(runtime.turned_off()[0].feature, "chat heads");
        assert!(RenderRuntime::is_installed());
        HeadsMod::disable_with(&host, &runtime);
    }
}

#[test]
fn a_chat_hook_that_fails_comes_out_and_the_locator_bar_stays() {
    let host = FakeHost::for_2660().with_failing_hook(HeadsEntries::BITMAP_FONT_GET_CHAR_WIDTH);
    let runtime = runtime();

    HeadsMod::enable_with(&host, &runtime).unwrap();

    let unhooked: Vec<u64> = host.unhooked().iter().map(|handle| handle.0).collect();
    assert_eq!(unhooked, [7]);
    assert!(host.errors()[0].starts_with("features off: chat heads"));
    assert!(RenderRuntime::is_installed());

    HeadsMod::disable_with(&host, &runtime);
    let unhooked: Vec<u64> = host.unhooked().iter().map(|handle| handle.0).collect();
    assert_eq!(unhooked, [7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn refuses_when_mesh_size_is_not_2660() {
    let host = FakeHost::for_2660().with_entry(HeadsEntries::MESH_SIZE, 0x278);
    let runtime = runtime();

    HeadsMod::enable_with(&host, &runtime).unwrap();

    assert!(host.hooked().is_empty());
    assert!(!RenderRuntime::is_installed());
    let warnings = host.warnings();
    assert_eq!(warnings.len(), 1);
    assert!(host.infos().is_empty());
    assert!(warnings[0].contains("mesh_size"));
    assert!(warnings[0].contains("0x278"));
}

#[test]
fn refuses_when_an_entry_is_missing() {
    let host = FakeHost::for_2660().without_entry(HeadsEntries::TEXTURE_GROUP_UNLOAD_TEXTURE);

    let result = HeadsMod::enable_with(&host, &runtime());

    assert!(
        matches!(result, Err(SdkError::MissingEntry(id)) if id == HeadsEntries::TEXTURE_GROUP_UNLOAD_TEXTURE.as_str())
    );
    assert!(host.hooked().is_empty());
    assert!(!RenderRuntime::is_installed());
}

#[test]
fn a_missing_layout_entry_installs_nothing() {
    for id in HeadsEntries::LAYOUT {
        let host = FakeHost::for_2660().without_entry(id);

        let result = HeadsMod::enable_with(&host, &runtime());

        assert!(matches!(&result, Err(SdkError::MissingEntry(missing)) if missing == id.as_str()), "{}", id.as_str());
        assert!(host.hooked().is_empty(), "{}", id.as_str());
        assert!(!RenderRuntime::is_installed(), "{}", id.as_str());
    }
}

fn error_for(feature: &str, id: EntryId) -> ErrorLine {
    ErrorLine {
        message: "game entry missing or unusable, feature turned off".to_string(),
        feature: feature.to_string(),
        entry: id.as_str().to_string(),
    }
}

// the faces and the chat heads install in full and only the named side feature is off
fn assert_only_off(host: &FakeHost, runtime: &HeadsRuntime, feature: &'static str, id: EntryId) {
    let mut expected = LOCATOR_BAR_HOOKS.to_vec();
    expected.extend([HeadsEntries::GUI_MESSAGE_CTOR.as_str(), HeadsEntries::BITMAP_FONT_GET_CHAR_WIDTH.as_str()]);
    assert_eq!(host.hooked(), expected, "{}", id.as_str());
    assert!(RenderRuntime::is_installed(), "{}", id.as_str());
    assert_eq!(runtime.link_state(), LinkState::Waiting, "{}", id.as_str());
    assert_eq!(host.infos(), [format!("features off: {feature}")], "{}", id.as_str());
    assert!(host.warnings().is_empty());
    let errors = host.errors();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains(id.as_str()) && errors[0].contains(feature), "{}", errors[0]);
    assert_eq!(runtime.turned_off(), [TurnedOff::new(feature, id.as_str())], "{}", id.as_str());
}

#[test]
fn a_missing_skin_diagnostics_entry_turns_off_only_the_diagnostics() {
    for id in HeadsEntries::DIAGNOSTICS {
        let host = FakeHost::for_2660().without_entry(id);
        let runtime = runtime();

        HeadsMod::enable_with(&host, &runtime).unwrap();

        assert_only_off(&host, &runtime, "skin diagnostics", id);
        HeadsMod::disable_with(&host, &runtime);
    }
}

#[test]
fn a_missing_geometry_entry_turns_off_only_the_geometry_head_locator() {
    for id in HeadsEntries::GEOMETRY {
        let host = FakeHost::for_2660().without_entry(id);
        let runtime = runtime();

        HeadsMod::enable_with(&host, &runtime).unwrap();

        assert_only_off(&host, &runtime, "geometry head locator", id);
        HeadsMod::disable_with(&host, &runtime);
    }
}

#[test]
fn a_missing_persona_entry_turns_off_only_the_persona_animated_faces() {
    for id in HeadsEntries::PERSONA {
        let host = FakeHost::for_2660().without_entry(id);
        let runtime = runtime();

        HeadsMod::enable_with(&host, &runtime).unwrap();

        assert_only_off(&host, &runtime, "persona animated faces", id);
        HeadsMod::disable_with(&host, &runtime);
    }
}

#[test]
fn a_persona_layout_the_mod_cannot_use_turns_off_only_the_persona_animated_faces() {
    // the image member would reach past the end of one element
    let id = HeadsEntries::ANIMATED_IMAGE_DATA_SIZE;
    let host = FakeHost::for_2660().with_entry(id, 0x20);
    let runtime = runtime();

    HeadsMod::enable_with(&host, &runtime).unwrap();

    assert_only_off(&host, &runtime, "persona animated faces", id);
    HeadsMod::disable_with(&host, &runtime);
}

#[test]
fn every_side_entry_missing_names_each_entry_and_the_faces_still_install() {
    FakeLog::install();
    let side: Vec<EntryId> =
        HeadsEntries::DIAGNOSTICS.into_iter().chain(HeadsEntries::GEOMETRY).chain(HeadsEntries::PERSONA).collect();
    let mut host = FakeHost::for_2660();
    for id in &side {
        host = host.without_entry(*id);
    }
    let runtime = runtime();

    HeadsMod::enable_with(&host, &runtime).unwrap();

    assert_eq!(host.hooked().len(), 8);
    assert!(RenderRuntime::is_installed());
    assert_eq!(host.infos(), ["features off: skin diagnostics, geometry head locator, persona animated faces"]);
    assert_eq!(host.errors().len(), side.len());
    let held = runtime.turned_off();
    for id in &side {
        assert!(held.iter().any(|item| item.entry == id.as_str()), "{}", id.as_str());
        assert!(host.errors().iter().any(|line| line.contains(id.as_str())), "{}", id.as_str());
    }
    HeadsMod::disable_with(&host, &runtime);
}

#[test]
fn the_held_features_reach_the_manager_log_once_when_the_link_attaches() {
    FakeLog::install();
    let id = HeadsEntries::GEOMETRY[0];
    let host = FakeHost::for_2660().without_entry(id);
    let runtime = runtime();
    HeadsMod::enable_with(&host, &runtime).unwrap();
    assert!(FakeLog::errors().is_empty());
    assert_eq!(runtime.turned_off().len(), 1);

    assert_eq!(runtime.attach_with(Ok(FakeTable::table())), LinkState::Attached);

    assert_eq!(FakeLog::errors(), [error_for("geometry head locator", id)]);
    assert!(runtime.turned_off().is_empty());
    runtime.poll_link();
    assert_eq!(FakeLog::errors().len(), 1);
    HeadsMod::disable_with(&host, &runtime);
}

#[test]
fn a_failed_attach_keeps_the_held_features_for_nothing_and_logs_no_curia_error() {
    FakeLog::install();
    let host = FakeHost::for_2660().without_entry(HeadsEntries::GEOMETRY[0]);
    let runtime = runtime();
    HeadsMod::enable_with(&host, &runtime).unwrap();

    assert_eq!(runtime.attach_with(Err(LinkError::ManagerMissing)), LinkState::Off);

    assert!(FakeLog::errors().is_empty());
    HeadsMod::disable_with(&host, &runtime);
}

#[test]
fn a_layout_the_mod_cannot_use_installs_nothing() {
    let cases = [
        (HeadsEntries::RESOURCE_LOCATION_SIZE, 0x40),
        (HeadsEntries::IMAGE_BUFFER_SIZE, 0x1000),
        (HeadsEntries::IMAGE_BUFFER_BYTE_SIZE, 0x30),
        (HeadsEntries::CLIENT_WAYPOINT_COLOR_ENGAGED, 0x40),
    ];
    for (id, value) in cases {
        let host = FakeHost::for_2660().with_entry(id, value);

        let result = HeadsMod::enable_with(&host, &runtime());

        assert!(matches!(result, Err(SdkError::Mod(_))), "{}", id.as_str());
        assert!(host.hooked().is_empty(), "{}", id.as_str());
        assert!(!RenderRuntime::is_installed(), "{}", id.as_str());
    }
}

#[test]
fn a_font_handle_layout_the_check_cannot_use_turns_off_only_chat_heads() {
    let host = FakeHost::for_2660().with_entry(HeadsEntries::FONT_HANDLE_DEFAULT_FONT, 0x50);
    let runtime = runtime();

    HeadsMod::enable_with(&host, &runtime).unwrap();

    assert_eq!(host.hooked(), LOCATOR_BAR_HOOKS);
    let errors = host.errors();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].starts_with("features off: chat heads"), "{}", errors[0]);
    assert!(errors[0].contains("FontHandle"), "{}", errors[0]);
    assert!(RenderRuntime::is_installed());
    HeadsMod::disable_with(&host, &runtime);
}

#[test]
fn a_vtable_slot_that_cannot_be_read_installs_nothing() {
    let host = FakeHost::for_2660().with_entry(HeadsEntries::HANDLE_PLAYER_SKIN_PACKET, 108);

    assert!(HeadsMod::enable_with(&host, &runtime()).is_err());
    assert!(host.hooked().is_empty());
    assert!(!RenderRuntime::is_installed());
}

#[test]
fn disable_removes_the_hooks_newest_first_then_drops_the_state() {
    let host = FakeHost::for_2660();
    let runtime = runtime();
    HeadsMod::enable_with(&host, &runtime).unwrap();

    HeadsMod::disable_with(&host, &runtime);

    let handles: Vec<u64> = host.unhooked().iter().map(|handle| handle.0).collect();
    assert_eq!(handles, [8, 7, 6, 5, 4, 3, 2, 1]);
    assert!(!RenderRuntime::is_installed());
    assert_eq!(runtime.link_state(), LinkState::Disabled);
}

#[test]
fn settings_change_applies_live() {
    let host = FakeHost::for_2660();
    let runtime = runtime();
    HeadsMod::enable_with(&host, &runtime).unwrap();
    assert_eq!(runtime.attach_with(Ok(FakeTable::table())), LinkState::Attached);
    assert_eq!(RenderRuntime::icon(0).1, 1.0);

    FakeTable::set_config("heads {\n  scale = 1.5\n}\n");
    FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });
    runtime.poll_link();

    assert_eq!(RenderRuntime::icon(0).1, 1.5);
}

#[test]
fn settings_are_read_when_the_link_attaches() {
    let host = FakeHost::for_2660();
    let runtime = runtime();
    HeadsMod::enable_with(&host, &runtime).unwrap();
    FakeTable::set_config("heads {\n  scale = 2.0\n}\n");

    runtime.attach_with(Ok(FakeTable::table()));

    assert_eq!(RenderRuntime::icon(0).1, 2.0);
}

#[test]
fn runs_with_defaults_when_manager_missing() {
    let host = FakeHost::for_2660();
    let runtime = runtime();
    HeadsMod::enable_with(&host, &runtime).unwrap();

    assert_eq!(runtime.attach_with(Err(LinkError::ManagerMissing)), LinkState::Off);

    assert_eq!(host.hooked().len(), 8);
    assert!(RenderRuntime::is_installed());
    assert_eq!(RenderRuntime::icon(0x1234).1, 1.0);
}

#[test]
fn manifest_lists_every_entry_the_mod_resolves() {
    let manifest = include_str!("../amml.hcl");

    for id in HeadsEntries::MANIFEST {
        assert!(manifest.contains(&format!("\"{}\"", id.as_str())), "{} is not in amml.hcl", id.as_str());
    }
    assert_eq!(manifest.matches("  \"").count(), HeadsEntries::MANIFEST.len());
}

#[test]
fn the_mod_id_is_the_namespaced_id() {
    assert_eq!(HeadsMod::id(), "@alaydriem/heads");
    assert_eq!(HeadsMod::id(), HeadsMod::MODULE);
}
