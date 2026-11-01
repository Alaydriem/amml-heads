use alaydriem_heads::structs::IconChoice;
use heads_core::config::HeadsSettings;
use heads_core::structs::FaceUpdate;

use crate::support::{FakeGame, FakeRig, GameCall, WaypointSpec};

const FACE_PIXEL: [u8; 4] = [9, 8, 7, 6];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

fn rig(specs: &[WaypointSpec]) -> FakeRig {
    FakeGame::take_log();
    FakeGame::group_available(true);
    FakeRig::new(specs, HeadsSettings::default())
}

fn is_face(choice: IconChoice) -> bool {
    matches!(choice, IconChoice::Face(ptr) if !ptr.is_null())
}

#[test]
fn entity_waypoint_with_face_decodes_to_face() {
    let mut rig = rig(&[WaypointSpec::plain(Some(RED)), WaypointSpec::player(42, Some(RED))]);
    rig.add_face(1, 42, FACE_PIXEL);

    rig.state.before_render(rig.client.addr());

    assert!(is_face(rig.state.icon_for(rig.waypoints.current_packed(1))));
    assert!(!is_face(rig.state.icon_for(rig.waypoints.current_packed(0))));
    assert!(FakeGame::take_log().contains(&GameCall::Upload { group: FakeGame::GROUP, slot: 0, pixel: FACE_PIXEL }));
    rig.state.after_render();
}

#[test]
fn entity_waypoint_without_a_face_stays_vanilla() {
    let mut rig = rig(&[WaypointSpec::player(42, Some(RED))]);

    rig.state.before_render(rig.client.addr());

    assert_eq!(rig.state.icon_for(rig.waypoints.current_packed(0)), IconChoice::Vanilla(0xFFFF0000));
    rig.state.after_render();
}

#[test]
fn plain_waypoint_decodes_to_vanilla_packed_colour() {
    let mut rig = rig(&[WaypointSpec::plain(Some([0.5, 0.25, 0.999, 1.0]))]);

    rig.state.before_render(rig.client.addr());
    let choice = rig.state.icon_for(rig.waypoints.current_packed(0));

    assert_eq!(choice, IconChoice::Vanilla(0xFF7F3FFE));
    rig.state.after_render();
}

#[test]
fn empty_colour_flag_decodes_to_white() {
    let mut rig = rig(&[WaypointSpec::plain(None)]);

    rig.state.before_render(rig.client.addr());
    let choice = rig.state.icon_for(rig.waypoints.current_packed(0));

    assert_eq!(choice, IconChoice::Vanilla(0xFFFF_FFFF));
    rig.state.after_render();
}

#[test]
fn after_render_restores_the_waypoint_bytes() {
    let mut rig = rig(&[WaypointSpec::plain(None), WaypointSpec::player(42, Some(RED))]);
    let before = [rig.waypoints.bytes(0), rig.waypoints.bytes(1)];

    rig.state.before_render(rig.client.addr());
    assert_ne!(rig.waypoints.bytes(0), before[0]);
    rig.state.after_render();

    assert_eq!([rig.waypoints.bytes(0), rig.waypoints.bytes(1)], before);
    assert_eq!(rig.state.icon_for(0x1234_5678), IconChoice::Vanilla(0x1234_5678));
}

#[test]
fn over_capacity_frame_tags_nothing_and_passes_through() {
    let mut rig = rig(&vec![WaypointSpec::plain(Some(RED)); 257]);
    let before = rig.waypoints.bytes(0);

    rig.state.before_render(rig.client.addr());

    assert_eq!(rig.waypoints.bytes(0), before);
    for packed in [0xFFA5_0000, 0xFFA5_0001, 0xFFFF_FFFF, 0x1234_5678] {
        assert_eq!(rig.state.icon_for(packed), IconChoice::Vanilla(packed));
    }
    rig.state.after_render();
}

#[test]
fn remove_during_frame_keeps_texture_until_frame_end() {
    let mut rig = rig(&[WaypointSpec::player(42, Some(RED))]);
    rig.add_face(1, 42, FACE_PIXEL);
    rig.state.before_render(rig.client.addr());
    let packed = rig.waypoints.current_packed(0);
    FakeGame::take_log();

    rig.sender.send(FaceUpdate::Remove { uuid: [1; 16] }).unwrap();
    assert!(is_face(rig.state.icon_for(packed)));
    assert_eq!(rig.state.deferred_len(), 0);
    rig.state.after_render();
    assert!(FakeGame::take_log().is_empty());

    rig.state.before_render(rig.client.addr());
    assert_eq!(rig.state.deferred_len(), 1);
    assert!(FakeGame::take_log().is_empty());
    assert!(!is_face(rig.state.icon_for(rig.waypoints.current_packed(0))));
    rig.state.after_render();

    assert_eq!(rig.state.deferred_len(), 0);
    assert_eq!(FakeGame::take_log(), vec![GameCall::Unload { group: FakeGame::GROUP, slot: 0 }]);
}

#[test]
fn disabled_setting_tags_nothing() {
    FakeGame::take_log();
    let mut rig = FakeRig::new(&[WaypointSpec::player(42, Some(RED))], HeadsSettings::new(false, 1.0, true));
    rig.add_face(1, 42, FACE_PIXEL);
    let before = rig.waypoints.bytes(0);

    rig.state.before_render(rig.client.addr());

    assert_eq!(rig.waypoints.bytes(0), before);
    assert_eq!(rig.state.icon_for(0xFFA5_0000), IconChoice::Vanilla(0xFFA5_0000));
    assert!(FakeGame::take_log().is_empty());
    rig.state.after_render();
}

#[test]
fn missing_texture_group_keeps_the_face_pending() {
    let mut rig = rig(&[WaypointSpec::player(42, Some(RED))]);
    rig.add_face(1, 42, FACE_PIXEL);
    FakeGame::group_available(false);

    rig.state.before_render(rig.client.addr());
    assert!(!is_face(rig.state.icon_for(rig.waypoints.current_packed(0))));
    rig.state.after_render();

    FakeGame::group_available(true);
    rig.state.before_render(rig.client.addr());
    assert!(is_face(rig.state.icon_for(rig.waypoints.current_packed(0))));
    rig.state.after_render();
}

#[test]
fn deferred_never_exceeds_the_name_pool() {
    let mut rig = rig(&[]);
    for n in 0..128u8 {
        rig.add_face(n, i64::from(n), FACE_PIXEL);
    }
    // the state applies 64 updates per frame
    for _ in 0..2 {
        rig.state.before_render(rig.client.addr());
        rig.state.after_render();
    }
    FakeGame::take_log();

    FakeGame::group_available(false);
    rig.sender.send(FaceUpdate::Clear).unwrap();
    for _ in 0..3 {
        rig.state.before_render(rig.client.addr());
        rig.state.after_render();
        assert_eq!(rig.state.deferred_len(), 128);
    }
    assert!(FakeGame::take_log().is_empty());

    FakeGame::group_available(true);
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();
    assert_eq!(rig.state.deferred_len(), 0);
    assert_eq!(FakeGame::take_log().len(), 128);
}

#[test]
fn exit_level_clears_store_and_unloads_textures() {
    let mut rig = rig(&[]);
    for n in 0..3u8 {
        rig.add_face(n, i64::from(n), FACE_PIXEL);
    }
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();
    assert_eq!(rig.state.face_count(), 3);
    assert_eq!(rig.state.locations_built(), 3);
    FakeGame::take_log();

    rig.sender.send(FaceUpdate::Clear).unwrap();
    rig.state.before_render(rig.client.addr());
    assert_eq!(rig.state.face_count(), 0);
    assert_eq!(rig.state.deferred_len(), 3);
    rig.state.after_render();

    let mut unloaded: Vec<u64> = FakeGame::take_log()
        .into_iter()
        .filter_map(|call| match call {
            GameCall::Unload { slot, .. } => Some(slot),
            _ => None,
        })
        .collect();
    unloaded.sort_unstable();
    assert_eq!(unloaded, vec![0, 1, 2]);
    assert_eq!(rig.state.deferred_len(), 0);
    assert_eq!(rig.state.locations_built(), 0);
}

#[test]
fn locations_are_kept_while_faces_remain_after_a_clear() {
    let mut rig = rig(&[]);
    rig.add_face(1, 1, FACE_PIXEL);
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();

    rig.sender.send(FaceUpdate::Clear).unwrap();
    rig.add_face(2, 2, FACE_PIXEL);
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();

    assert_eq!(rig.state.face_count(), 1);
    // the new face reuses the name the cleared face gave back
    assert_eq!(rig.state.locations_built(), 1);

    rig.sender.send(FaceUpdate::Clear).unwrap();
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();
    assert_eq!(rig.state.locations_built(), 0);
}

#[test]
fn request_clear_drops_queued_updates_and_clears_the_store() {
    let mut rig = rig(&[]);
    rig.add_face(1, 1, FACE_PIXEL);
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();
    rig.add_face(2, 2, FACE_PIXEL);

    rig.state.request_clear();
    rig.state.before_render(rig.client.addr());
    rig.state.after_render();

    assert_eq!(rig.state.face_count(), 0);
    assert_eq!(rig.state.deferred_len(), 0);
}

// every actor of `n` faces has a waypoint, so the test can ask what each one draws
fn players(first_actor: i64, count: i64) -> Vec<WaypointSpec> {
    (first_actor..first_actor + count).map(|actor| WaypointSpec::player(actor, Some(RED))).collect()
}

fn faces_drawn(rig: &mut FakeRig, count: usize) -> usize {
    (0..count).filter(|&index| is_face(rig.state.icon_for(rig.waypoints.current_packed(index)))).count()
}

#[test]
fn a_new_face_in_a_full_store_gets_a_texture_in_the_same_frame() {
    let mut rig = rig(&players(1000, 1));
    for n in 0..128u8 {
        rig.add_face(n, i64::from(n), FACE_PIXEL);
    }
    for _ in 0..2 {
        rig.state.before_render(rig.client.addr());
        rig.state.after_render();
    }
    assert_eq!(rig.state.face_count(), 128);

    rig.add_face(200, 1000, FACE_PIXEL);
    rig.state.before_render(rig.client.addr());

    assert_eq!(rig.state.face_count(), 128);
    assert_eq!(faces_drawn(&mut rig, 1), 1);
    rig.state.after_render();
}

#[test]
fn a_world_change_with_many_faces_keeps_every_new_face() {
    let mut rig = rig(&players(1000, 4));
    for n in 0..125u8 {
        rig.add_face(n, i64::from(n), FACE_PIXEL);
    }
    for _ in 0..2 {
        rig.state.before_render(rig.client.addr());
        rig.state.after_render();
    }
    assert_eq!(rig.state.face_count(), 125);

    rig.sender.send(FaceUpdate::Clear).unwrap();
    for n in 0..4u8 {
        rig.add_face(200 + n, 1000 + i64::from(n), FACE_PIXEL);
    }
    rig.state.before_render(rig.client.addr());

    assert_eq!(rig.state.face_count(), 4);
    assert_eq!(faces_drawn(&mut rig, 4), 4);
    rig.state.after_render();
}

#[test]
fn a_face_without_a_free_name_is_retried_on_the_next_frame() {
    let mut rig = rig(&players(1000, 1));
    for n in 0..128u8 {
        rig.add_face(n, i64::from(n), FACE_PIXEL);
    }
    for _ in 0..2 {
        rig.state.before_render(rig.client.addr());
        rig.state.after_render();
    }
    // the group is missing, so the released texture cannot be unloaded and its name stays taken
    FakeGame::group_available(false);
    rig.add_face(200, 1000, FACE_PIXEL);
    rig.state.before_render(rig.client.addr());
    assert_eq!(faces_drawn(&mut rig, 1), 0);
    rig.state.after_render();

    FakeGame::group_available(true);
    rig.state.before_render(rig.client.addr());
    assert_eq!(faces_drawn(&mut rig, 1), 1);
    rig.state.after_render();
}

#[test]
fn dropping_a_tagged_state_puts_the_colours_back() {
    let rig = rig(&[WaypointSpec::plain(Some(RED))]);
    let before = rig.waypoints.bytes(0);
    let FakeRig { mut state, waypoints, client, .. } = rig;
    state.before_render(client.addr());
    assert_ne!(waypoints.bytes(0), before);

    drop(state);

    assert_eq!(waypoints.bytes(0), before);
}
