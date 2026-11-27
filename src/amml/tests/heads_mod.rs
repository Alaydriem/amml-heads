use alaydriem_heads::HeadsMod;
use amm_client::link::LinkUses;
use amm_client::test_support::FakeTable;
use amml_api::amml_abi::{LocatorBarIconDraw, WorldLeft};
use amml_api::testing::FakeGame;
use amml_api::{ActorUniqueId, AmmlMod, Area, Rect, Rgba, TextureReload};

use crate::support::heads_rig::HeadsRig;
use crate::support::skins::Skins;

const MANIFEST: &str = include_str!("../amml.hcl");
const WHITE: Rgba = Rgba { r: 255, g: 255, b: 255, a: 255 };
const VANILLA_RECT: Rect = Rect { x: 10.0, y: 20.0, width: 8.0, height: 9.0 };

fn steve(actor: i64) -> amml_api::testing::FakePlayerEntry {
    HeadsRig::entry(HeadsRig::STEVE, actor, "Steve", Some(Skins::classic([10, 200, 30])))
}

fn alex(actor: i64) -> amml_api::testing::FakePlayerEntry {
    HeadsRig::entry(HeadsRig::ALEX, actor, "Alex", Some(Skins::classic([200, 10, 30])))
}

#[test]
fn enable_logs_the_features_the_loader_does_not_serve() {
    let rig = HeadsRig::new();

    assert!(rig.logged("info heads enabled features_off="));
}

#[test]
fn a_player_list_gives_each_face_a_texture() {
    let rig = HeadsRig::new();

    rig.add(vec![steve(7), alex(8)]);

    let uploads = rig.game.textures().uploads();
    assert_eq!(uploads.iter().map(|(_, width, height)| (*width, *height)).collect::<Vec<_>>(), [(32, 32), (32, 32)]);
    assert!(rig.logged("info heads player list action=add entries=2"));
    assert!(rig.logged("info heads face ready source=skin head=classic"));
    assert!(rig.logged("info heads face uploaded actor=7"));
}

#[test]
fn the_locator_bar_icon_of_a_player_is_its_face() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7), alex(8)]);
    let (texture, _, _) = rig.game.textures().uploads()[0];

    let edit = rig.icon(7);

    assert_eq!(edit.texture, Some(texture));
    assert_eq!(edit.rect, VANILLA_RECT);
    assert_eq!(edit.color, WHITE);
    assert!(rig.logged("info heads icon face drawn actor=7"));
}

#[test]
fn a_waypoint_that_is_not_a_player_with_a_face_keeps_the_vanilla_dot() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.icon(7);

    assert_eq!(rig.icon(99).texture, None);
    assert_eq!(rig.game.raise_locator_icon(None).texture, None);
}

#[test]
fn a_face_that_is_not_ready_keeps_the_vanilla_dot() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);

    let edit = rig.game.raise_locator_icon(Some(ActorUniqueId(7)));

    assert_eq!(edit.texture, None);
    assert_eq!(edit.rect, VANILLA_RECT);
    assert!(!rig.logged("info heads icon face drawn actor=7"));
}

#[test]
fn the_scale_setting_grows_the_face_about_its_centre() {
    let rig = HeadsRig::with_config(Some("heads {\n  scale = 2.0\n}\n"));
    rig.tick();
    rig.add(vec![steve(7)]);

    let edit = rig.icon(7);

    assert_eq!(edit.rect, Rect { x: 6.0, y: 15.5, width: 16.0, height: 18.0 });
    assert!(rig.logged("info heads settings applied enabled=true scale=2 chat=true"));
}

#[test]
fn a_settings_change_applies_to_the_next_icon() {
    let rig = HeadsRig::new();
    rig.tick();
    rig.add(vec![steve(7)]);
    assert_eq!(rig.icon(7).rect, VANILLA_RECT);

    rig.settings_changed("heads {\n  scale = 1.5\n}\n");

    assert_eq!(rig.icon(7).rect, Rect { x: 8.0, y: 17.75, width: 12.0, height: 13.5 });
    assert_eq!(rig.logged_count("info heads settings applied"), 2);
}

#[test]
fn a_scale_outside_the_range_is_clamped() {
    let rig = HeadsRig::with_config(Some("heads {\n  scale = 9.0\n}\n"));
    rig.tick();
    rig.add(vec![steve(7)]);

    assert_eq!(rig.icon(7).rect, Rect { x: 6.0, y: 15.5, width: 16.0, height: 18.0 });
}

#[test]
fn with_the_faces_off_nothing_is_uploaded_until_they_are_turned_on() {
    let rig = HeadsRig::with_config(Some("heads {\n  enabled = false\n}\n"));
    rig.tick();
    rig.add(vec![steve(7)]);

    assert!(rig.game.textures().uploads().is_empty());
    assert_eq!(rig.icon(7).texture, None);

    rig.settings_changed("heads {\n  enabled = true\n}\n");

    assert_eq!(rig.game.textures().uploads().len(), 1);
    assert!(rig.icon(7).texture.is_some());
}

#[test]
fn a_skin_change_replaces_the_texture_and_frees_the_old_one() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    let (first, _, _) = rig.game.textures().uploads()[0];

    rig.game.raise_skin(HeadsRig::STEVE, Some(Skins::classic([1, 2, 3])));

    let uploads = rig.game.textures().uploads();
    assert_eq!(uploads.len(), 2);
    assert_eq!(rig.game.textures().unloads(), vec![first]);
    assert_eq!(rig.icon(7).texture, Some(uploads[1].0));
    assert!(rig.logged("info heads skin changed uuid=11111111 source=skin"));
}

#[test]
fn a_skin_change_of_an_unknown_player_changes_nothing() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);

    rig.game.raise_skin(HeadsRig::ALEX, Some(Skins::classic([1, 2, 3])));

    assert_eq!(rig.game.textures().uploads().len(), 1);
    assert!(rig.game.textures().unloads().is_empty());
}

#[test]
fn a_player_that_leaves_the_list_loses_the_face_and_the_texture() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    let (texture, _, _) = rig.game.textures().uploads()[0];

    rig.remove(HeadsRig::STEVE);

    assert_eq!(rig.game.textures().unloads(), vec![texture]);
    assert_eq!(rig.icon(7).texture, None);
    assert!(rig.logged("info heads player list action=remove entries=1"));
}

#[test]
fn a_player_without_a_usable_skin_gets_no_face() {
    let rig = HeadsRig::new();

    rig.add(vec![
        HeadsRig::entry(HeadsRig::STEVE, 7, "Steve", Some(Skins::unreadable())),
        HeadsRig::entry(HeadsRig::ALEX, 8, "Alex", Some(Skins::empty())),
        HeadsRig::entry(amml_api::Uuid([0x33; 16]), 9, "Bob", None),
    ]);

    assert!(rig.game.textures().uploads().is_empty());
    assert_eq!(rig.icon(7).texture, None);
    assert!(rig.logged("info heads player list action=add entries=3"));
}

#[test]
fn a_persona_face_is_drawn_from_the_animated_face() {
    let rig = HeadsRig::new();

    rig.add(vec![HeadsRig::entry(HeadsRig::STEVE, 7, "Steve", Some(Skins::persona([0, 0, 255])))]);

    assert!(rig.logged("info heads face ready source=persona head=animated_face"));
    assert_eq!(rig.game.textures().uploads().len(), 1);
}

#[test]
fn leaving_the_world_clears_the_faces_and_frees_the_textures() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7), alex(8)]);
    rig.tick();

    rig.game.raise(&mut WorldLeft { header: FakeGame::header::<WorldLeft>(), epoch: 0 });

    assert_eq!(rig.game.textures().unloads().len(), 2);
    assert_eq!(rig.icon(7).texture, None);
    assert!(rig.game.icons().is_empty());
    assert!(rig.logged("info heads world left faces=0"));
    assert_eq!(rig.say("Steve", "hi"), (Some("<Steve> hi".to_string()), Some("<Steve> **".to_string())));
}

#[test]
fn a_list_past_the_store_capacity_keeps_128_faces() {
    let rig = HeadsRig::new();
    let entries = (0..130u8)
        .map(|n| {
            HeadsRig::entry(
                amml_api::Uuid([n + 1; 16]),
                i64::from(n),
                &format!("Player{n}"),
                Some(Skins::classic([n, 1, 2])),
            )
        })
        .collect();

    rig.add(entries);

    assert_eq!(rig.game.textures().uploads().len(), 128);
    assert!(rig.game.textures().unloads().is_empty());
}

#[test]
fn the_proof_lines_stop_after_their_budget() {
    let rig = HeadsRig::new();
    let entries = (0..40u8)
        .map(|n| {
            HeadsRig::entry(
                amml_api::Uuid([n + 1; 16]),
                i64::from(n),
                &format!("Player{n}"),
                Some(Skins::classic([n, 1, 2])),
            )
        })
        .collect();

    rig.add(entries);

    assert_eq!(rig.logged_count("info heads face ready"), 16);
    assert_eq!(rig.logged_count("info heads face uploaded"), 8);
}

#[test]
fn an_upload_the_loader_refuses_is_logged_once_and_tried_again_later() {
    let rig = HeadsRig::new();
    rig.game.withdraw_area(Area::UI_TEXTURES);
    rig.add(vec![steve(7)]);
    for _ in 0..3 {
        rig.tick();
    }
    assert!(rig.game.textures().uploads().is_empty());
    assert_eq!(rig.logged_count("warn heads face upload refused"), 1);

    rig.game.offer_area(Area::UI_TEXTURES);
    for _ in 0..70 {
        rig.tick();
    }

    assert_eq!(rig.game.textures().uploads().len(), 1);
    assert!(rig.icon(7).texture.is_some());
}

#[test]
fn a_chat_line_of_a_player_with_a_face_gets_the_face_glyph() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.tick();

    let (text, filtered) = rig.say("Steve", "hi");

    assert_eq!(text.as_deref(), Some("\u{E110}<Steve> hi"));
    assert_eq!(filtered.as_deref(), Some("\u{E110}<Steve> **"));
    assert!(rig.logged("info heads chat icon registered glyph=U+E110"));
    assert!(rig.logged("info heads chat line edited kind=1"));
}

#[test]
fn a_chat_line_of_another_player_is_left_alone() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.tick();

    let (text, _) = rig.say("Mallory", "hi");

    assert_eq!(text.as_deref(), Some("<Mallory> hi"));
}

#[test]
fn a_chat_line_gets_no_glyph_before_the_icon_is_registered() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);

    let (text, _) = rig.say("Steve", "hi");

    assert_eq!(text.as_deref(), Some("<Steve> hi"));
}

#[test]
fn the_chat_setting_turns_the_face_glyph_off_and_strips_forged_ones() {
    let rig = HeadsRig::with_config(Some("heads {\n  chat = false\n}\n"));
    rig.tick();
    rig.add(vec![steve(7)]);
    rig.tick();

    let (text, _) = rig.say("Steve", "hi");
    let (forged, _) = rig.say("Steve", "\u{E115}hi");

    assert_eq!(text.as_deref(), Some("<Steve> hi"));
    assert_eq!(forged.as_deref(), Some("<Steve> hi"));
}

#[test]
fn the_chat_setting_turns_the_face_glyph_on_again_live() {
    let rig = HeadsRig::with_config(Some("heads {\n  chat = false\n}\n"));
    rig.tick();
    rig.add(vec![steve(7)]);
    rig.tick();
    assert_eq!(rig.say("Steve", "hi").0.as_deref(), Some("<Steve> hi"));

    rig.settings_changed("heads {\n  chat = true\n}\n");

    assert_eq!(rig.say("Steve", "hi").0.as_deref(), Some("\u{E110}<Steve> hi"));
}

#[test]
fn a_font_that_is_not_a_bitmap_font_gets_no_glyph_until_it_is_one() {
    let rig = HeadsRig::new();
    rig.game.set_bitmap_font(false);
    rig.add(vec![steve(7)]);
    rig.tick();
    assert!(rig.game.icons().is_empty());
    assert_eq!(rig.say("Steve", "hi").0.as_deref(), Some("<Steve> hi"));

    rig.game.set_bitmap_font(true);
    for _ in 0..70 {
        rig.tick();
    }

    assert_eq!(rig.game.icons().len(), 1);
    assert_eq!(rig.say("Steve", "hi").0.as_deref(), Some("\u{E110}<Steve> hi"));
}

#[test]
fn a_player_without_a_face_gets_no_chat_glyph() {
    let rig = HeadsRig::new();
    rig.add(vec![HeadsRig::entry(HeadsRig::STEVE, 7, "Steve", Some(Skins::unreadable()))]);
    rig.tick();

    assert_eq!(rig.say("Steve", "hi").0.as_deref(), Some("<Steve> hi"));
    assert!(rig.game.icons().is_empty());
}

#[test]
fn a_skin_change_updates_the_chat_icon_in_place() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.tick();

    rig.game.raise_skin(HeadsRig::STEVE, Some(Skins::classic([1, 2, 3])));
    rig.tick();

    assert_eq!(rig.game.icons(), vec!['\u{E110}']);
    assert_eq!(rig.say("Steve", "hi").0.as_deref(), Some("\u{E110}<Steve> hi"));
}

#[test]
fn a_join_line_names_the_player_with_the_face() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.tick();

    let (text, _) = rig.game.raise_chat(&amml_api::testing::FakeChat {
        kind: amml_api::ChatKind(3),
        text: "Steve joined the game".to_string(),
        ..amml_api::testing::FakeChat::default()
    });

    assert_eq!(text.as_deref(), Some("\u{E110}Steve joined the game"));
}

#[test]
fn without_the_manager_the_defaults_apply_and_no_settings_line_is_logged() {
    let rig = HeadsRig::with_config(None);
    rig.tick();

    rig.add(vec![steve(7)]);

    assert_eq!(rig.icon(7).texture, Some(rig.game.textures().uploads()[0].0));
    assert_eq!(rig.icon(7).rect, VANILLA_RECT);
    assert_eq!(rig.logged_count("info heads settings applied"), 0);
}

#[test]
fn disable_removes_the_listeners_and_frees_the_icons() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.tick();
    assert_eq!(rig.game.icons().len(), 1);

    rig.game.disable::<HeadsMod>();

    assert!(rig.game.icons().is_empty());
    assert_eq!(rig.icon(7).texture, None);
    let calls = FakeTable::calls();
    assert!(calls.contains(&"unregister"), "{calls:?}");
}

#[test]
fn a_mod_that_is_enabled_again_starts_with_no_faces() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    rig.game.disable::<HeadsMod>();

    rig.game.enable::<HeadsMod>().unwrap();

    assert_eq!(rig.icon(7).texture, None);
}

#[test]
fn a_refused_required_listen_is_logged_and_fails_enable() {
    let game = HeadsRig::served_game();
    game.refuse_listen::<LocatorBarIconDraw>();
    FakeTable::provide(&game.ctx()).unwrap();
    game.load::<HeadsMod>().unwrap();

    assert!(game.enable::<HeadsMod>().is_err());
    let logs = game.logs();
    assert!(logs.iter().any(|line| line.starts_with("error heads off error=")), "{logs:?}");
}

#[test]
fn the_manifest_names_the_mod_and_lists_no_entries() {
    assert!(MANIFEST.contains("namespace    = \"alaydriem\""));
    assert!(MANIFEST.contains("name         = \"heads\""));
    assert!(MANIFEST.contains("api_version  = 3"));
    assert!(!MANIFEST.contains("entries"));
}

#[test]
fn every_row_the_mod_uses_is_required_but_the_ones_of_the_manager_link() {
    let optional: Vec<&str> = HeadsMod::USES
        .rows()
        .iter()
        .filter(|row| row.required == 0)
        // SAFETY: the rows of a `Uses` are built by `amml_uses!` from `&'static str` names
        .map(|row| unsafe { row.name.as_str() }.unwrap())
        .collect();

    assert_eq!(
        optional,
        [
            "input.lease_suspended",
            "input.lease_resumed",
            "input.lease_lost",
            "input.injected_released",
            "overlay.screenshot_taken",
            "overlay.screenshot_written",
            "input.lease"
        ]
    );
}

#[test]
fn link_uses_are_complete() {
    assert_eq!(LinkUses::check(&HeadsMod::USES), Ok(()));
}

#[test]
fn the_manifest_uses_block_is_the_uses_of_the_mod() {
    let manifest = MANIFEST.replace("\r\n", "\n");

    assert_eq!(manifest.matches(&HeadsMod::USES.to_hcl()).count(), 1);
    assert_eq!(HeadsMod::USES.write_into(&manifest), manifest);
}

#[test]
fn the_manifest_writer_keeps_a_crlf_manifest_and_adds_no_second_block() {
    let crlf = MANIFEST.replace("\r\n", "\n").replace('\n', "\r\n");

    let once = HeadsMod::USES.write_into(&crlf);
    let twice = HeadsMod::USES.write_into(&once);

    assert_eq!(once, crlf);
    assert_eq!(twice, crlf);
    assert_eq!(twice.matches("\nuses {\r\n").count(), 1);
}

#[test]
fn a_device_reset_uploads_every_face_again_and_the_icon_draws_the_new_texture() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7), alex(8)]);
    let old = rig.game.textures().uploads().len();
    assert_eq!(old, 2);

    rig.game.raise_textures_reloaded(TextureReload::DeviceReset);

    let uploads = rig.game.textures().uploads();
    assert_eq!(uploads.len(), 4);
    assert!(rig.logged("info heads textures reloaded reason=device_reset reuploaded=2"));
    let edit = rig.icon(7);
    assert!(edit.texture.is_some());
    assert!(uploads[2..].iter().any(|(texture, _, _)| Some(*texture) == edit.texture));
    assert!(rig.game.textures().unloads().is_empty());
}

#[test]
fn a_resource_pack_change_keeps_the_faces_and_uploads_nothing() {
    let rig = HeadsRig::new();
    rig.add(vec![steve(7)]);
    let (texture, _, _) = rig.game.textures().uploads()[0];

    rig.game.raise_textures_reloaded(TextureReload::ResourcePack);

    assert_eq!(rig.game.textures().uploads().len(), 1);
    assert_eq!(rig.icon(7).texture, Some(texture));
    assert!(rig.logged("info heads textures reloaded reason=resource_pack reuploaded=0"));
}

#[test]
fn a_device_reset_with_the_faces_off_uploads_when_they_are_turned_on() {
    let rig = HeadsRig::new();
    rig.tick();
    rig.add(vec![steve(7)]);
    rig.settings_changed(
        "heads {
  enabled = false
}
",
    );

    rig.game.raise_textures_reloaded(TextureReload::DeviceReset);

    assert_eq!(rig.game.textures().uploads().len(), 1);
    assert!(rig.logged("info heads textures reloaded reason=device_reset reuploaded=0"));
    rig.settings_changed(
        "heads {
  enabled = true
}
",
    );
    assert_eq!(rig.game.textures().uploads().len(), 2);
    assert!(rig.icon(7).texture.is_some());
}
