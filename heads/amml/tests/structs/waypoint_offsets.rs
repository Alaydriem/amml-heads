use alaydriem_heads::structs::{SavedWaypoint, WaypointOffsets};

fn offsets(color: usize, engaged: usize) -> Option<WaypointOffsets> {
    WaypointOffsets::new(0x10, 0x20, color, engaged, 0x60, 0x78)
}

#[test]
fn the_engaged_byte_must_lie_in_the_saved_colour_bytes() {
    assert!(offsets(0x18, 0x28).is_some());
    assert!(offsets(0x18, 0x18).is_some());
    assert!(offsets(0x18, 0x18 + SavedWaypoint::COLOR_BYTES - 1).is_some());
    assert!(offsets(0x18, 0x18 + SavedWaypoint::COLOR_BYTES).is_none());
    assert!(offsets(0x18, 0x10).is_none());
}

#[test]
fn a_colour_at_the_end_of_the_address_space_is_refused() {
    assert!(offsets(usize::MAX - 4, usize::MAX - 2).is_none());
}
