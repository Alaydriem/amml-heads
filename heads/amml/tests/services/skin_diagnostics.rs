use alaydriem_heads::services::{PacketObserver, SkinDiagnostics};
use alaydriem_heads::structs::{PacketOffsets, SkinReport};
use crossbeam_channel::bounded;

use crate::support::{FakeEntry, FakePacket, FakeSkin};

fn offsets() -> PacketOffsets {
    PacketOffsets::new(0xD8, 0xD0, 0xA8, 0x80, 0xF8)
}

fn many_players(count: u8, skin: &FakeSkin) -> FakePacket {
    let entries: Vec<FakeEntry> =
        (0..count).map(|n| FakeEntry { variant: 1, uuid: [n; 16], actor: i64::from(n), skin: Some(skin) }).collect();
    FakePacket::player_list(&entries)
}

fn observer(diagnostics: SkinDiagnostics) -> PacketObserver {
    let (sender, receiver) = bounded(256);
    // the receiver stays open for the test
    std::mem::forget(receiver);
    PacketObserver::new_with_diagnostics(sender, offsets(), diagnostics)
}

#[test]
fn inspect_reads_flags_strings_and_animated_images() {
    let mut skin = FakeSkin::rgba(64, 64, [1, 2, 3, 255]);
    skin.mark(false, true);
    skin.set_strings("geometry.custom", 120);
    skin.set_animated(&[1, 2]);

    // SAFETY: the fake skin lives for the call and has the 26.60 layout.
    let report = unsafe { SkinDiagnostics::inspect(skin.skin_ref(), &offsets()) }.unwrap();

    assert_eq!(
        report,
        SkinReport {
            width: 64,
            height: 64,
            format: 4,
            blob_size: 64 * 64 * 4,
            size_matches: true,
            is_persona: true,
            is_premium: false,
            default_geometry: "geometry.custom".to_string(),
            resource_patch_len: 120,
            animated_types: vec![1, 2],
        }
    );
}

#[test]
fn inspect_reports_a_size_mismatch_and_survives_a_null_ref() {
    let skin = FakeSkin::new(128, 128, 4, 100, [1, 2, 3, 255]);

    // SAFETY: the fake skin lives for the call; a null reference is rejected before any read.
    let report = unsafe { SkinDiagnostics::inspect(skin.skin_ref(), &offsets()) };
    // SAFETY: a null reference is rejected before any read.
    let none = unsafe { SkinDiagnostics::inspect(0, &offsets()) };

    assert!(!report.unwrap().size_matches);
    assert!(none.is_none());
}

#[test]
fn logs_at_most_64_players() {
    let skin = FakeSkin::rgba(64, 64, [1, 2, 3, 255]);
    let observer = observer(SkinDiagnostics::new());
    let packet = many_players(70, &skin);

    // SAFETY: the fake packet and skin live for the calls and match the 26.60 layout.
    unsafe {
        observer.on_player_list(packet.addr());
        observer.on_player_list(packet.addr());
    }

    assert_eq!(observer.diagnostics().logged(), SkinDiagnostics::LOG_LIMIT);
}

#[test]
fn a_repeated_player_is_logged_once() {
    let skin = FakeSkin::rgba(64, 64, [1, 2, 3, 255]);
    let observer = observer(SkinDiagnostics::new());
    let packet = many_players(1, &skin);

    // SAFETY: the fake packet and skin live for the calls and match the 26.60 layout.
    unsafe {
        observer.on_player_list(packet.addr());
        observer.on_player_list(packet.addr());
    }

    assert_eq!(observer.diagnostics().logged(), 1);
}
