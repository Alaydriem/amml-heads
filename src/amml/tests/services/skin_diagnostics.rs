use std::sync::Arc;

use alaydriem_heads::services::{FaceReader, SkinDiagnostics};
use amml_api::amml_abi::PlayerSkinChanged;
use amml_api::testing::{FakeGame, FakeSkin};
use amml_api::{HookOrder, PlayerSkinChangedExt, Uuid};

use crate::support::heads_rig::HeadsRig;
use crate::support::skins::Skins;

// raises a skin change for each `(uuid byte, skin)` and records it with the diagnostics of the game
fn record_all(skins: Vec<(u8, FakeSkin)>) -> (FakeGame, Arc<SkinDiagnostics>) {
    let game = HeadsRig::served_game();
    let diagnostics = Arc::new(SkinDiagnostics::new(game.ctx().log()));
    let sink = diagnostics.clone();
    let _listener = game
        .ctx()
        .events()
        .listen::<PlayerSkinChanged>(HookOrder::NORMAL, move |event, _| {
            if let (Some(uuid), Some(skin)) = (event.uuid(), event.skin()) {
                let read = FaceReader::read(&skin);
                sink.record("PlayerSkin", uuid.0, None, &skin, &read);
            }
        })
        .unwrap();
    for (byte, skin) in skins {
        game.raise_skin(Uuid([byte; 16]), Some(skin));
    }
    (game, diagnostics)
}

#[test]
fn a_player_is_logged_once_with_what_the_skin_holds() {
    let (game, diagnostics) = record_all(vec![(1, Skins::persona([0, 0, 255])), (1, Skins::persona([0, 0, 255]))]);

    assert_eq!(diagnostics.logged(), 1);
    let logs = game.logs();
    let line = logs.iter().find(|line| line.starts_with("info heads skin read ")).unwrap();
    for field in ["kind=PlayerSkin", "uuid=01010101", "outcome=Ok", "is_persona=true", "animated_types=1", "width=64"] {
        assert!(line.contains(field), "{field} in {line}");
    }
}

#[test]
fn the_log_stops_at_64_players() {
    let skins = (0..70).map(|byte| (byte, Skins::classic([1, 2, 3]))).collect();

    let (game, diagnostics) = record_all(skins);

    assert_eq!(diagnostics.logged(), SkinDiagnostics::LOG_LIMIT);
    assert_eq!(game.logs().iter().filter(|line| line.starts_with("info heads skin read ")).count(), 64);
}

#[test]
fn a_skin_without_an_image_is_logged_as_unreadable() {
    let (game, _) = record_all(vec![(3, Skins::unreadable())]);

    let logs = game.logs();
    let line = logs.iter().find(|line| line.starts_with("info heads skin read ")).unwrap();
    assert!(line.contains("unreadable=true") && line.contains("outcome=SkinNotReadable"), "{line}");
}
