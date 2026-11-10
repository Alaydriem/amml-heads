use alaydriem_heads::services::WaypointTagger;
use alaydriem_heads::structs::TagFrame;

use crate::support::{ENTITY_VTABLE, FakeLayout, FakeWaypoints, WaypointSpec};

fn tag(list: &FakeWaypoints, frame: &mut TagFrame) -> bool {
    // SAFETY: the fake list is a live circular list of fake waypoints.
    unsafe { WaypointTagger::tag(list.head(), ENTITY_VTABLE, &FakeLayout::waypoints(), frame) }
}

fn restore(frame: &mut TagFrame) {
    // SAFETY: the fake waypoints are still alive.
    unsafe { WaypointTagger::restore(frame, &FakeLayout::waypoints()) };
}

fn all_bytes(list: &FakeWaypoints) -> Vec<Vec<u8>> {
    (0..list.len()).map(|i| list.bytes(i)).collect()
}

#[test]
fn tags_every_visible_waypoint_and_restores_bytes_exactly() {
    let list = FakeWaypoints::new(&[
        WaypointSpec::plain(Some([0.2, 0.4, 0.6, 1.0])),
        WaypointSpec::plain(None),
        WaypointSpec::player(7, Some([1.0, 0.0, 0.0, 1.0])),
        WaypointSpec::hidden(),
    ]);
    let before = all_bytes(&list);
    let mut frame = TagFrame::new();

    assert!(tag(&list, &mut frame));
    assert!(frame.is_active());
    assert_eq!(frame.count(), 3);
    let during = all_bytes(&list);
    assert_ne!(during[0], before[0]);
    assert_ne!(during[1], before[1]);
    assert_ne!(during[2], before[2]);

    restore(&mut frame);
    assert_eq!(all_bytes(&list), before);
    assert!(!frame.is_active());
    assert_eq!(frame.count(), 0);
}

#[test]
fn invisible_waypoints_are_not_tagged() {
    let list = FakeWaypoints::new(&[WaypointSpec::hidden(), WaypointSpec::hidden()]);
    let before = all_bytes(&list);
    let mut frame = TagFrame::new();

    assert!(tag(&list, &mut frame));
    assert_eq!(frame.count(), 0);
    assert_eq!(all_bytes(&list), before);
}

#[test]
fn over_capacity_frame_tags_nothing() {
    let specs = vec![WaypointSpec::plain(Some([0.1, 0.2, 0.3, 1.0])); 257];
    let list = FakeWaypoints::new(&specs);
    let before = all_bytes(&list);
    let mut frame = TagFrame::new();

    assert!(!tag(&list, &mut frame));
    assert!(!frame.is_active());
    assert_eq!(frame.count(), 0);
    assert_eq!(all_bytes(&list), before);
}

#[test]
fn exactly_capacity_is_tagged() {
    let specs = vec![WaypointSpec::plain(Some([0.1, 0.2, 0.3, 1.0])); 256];
    let list = FakeWaypoints::new(&specs);
    let mut frame = TagFrame::new();

    assert!(tag(&list, &mut frame));
    assert_eq!(frame.count(), 256);
    restore(&mut frame);
}

#[test]
fn list_longer_than_the_walk_cap_tags_nothing() {
    let within = FakeWaypoints::new(&vec![WaypointSpec::hidden(); 1024]);
    let mut frame = TagFrame::new();
    assert!(tag(&within, &mut frame));

    let beyond = FakeWaypoints::new(&vec![WaypointSpec::hidden(); 1025]);
    assert!(!tag(&beyond, &mut frame));
    assert!(!frame.is_active());
}

#[test]
fn null_list_head_tags_nothing() {
    let mut frame = TagFrame::new();
    // SAFETY: a null head is rejected before any read.
    assert!(!unsafe { WaypointTagger::tag(0, ENTITY_VTABLE, &FakeLayout::waypoints(), &mut frame) });
}
