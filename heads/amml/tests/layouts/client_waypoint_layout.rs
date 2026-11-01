use std::mem::{offset_of, size_of};

use alaydriem_heads::layouts::ClientWaypointLayout;

use crate::support::FakeMemory;

#[test]
fn offsets_match_2660() {
    assert_eq!(offset_of!(ClientWaypointLayout, position), 0x08);
    assert_eq!(offset_of!(ClientWaypointLayout, color), 0x18);
    assert_eq!(offset_of!(ClientWaypointLayout, texture_path), 0x30);
    assert_eq!(offset_of!(ClientWaypointLayout, icon_size), 0x58);
    assert_eq!(offset_of!(ClientWaypointLayout, visible), 0x60);
    assert_eq!(size_of::<ClientWaypointLayout>(), 0x78);
}

#[test]
fn optional_flags_sit_at_2660_offsets() {
    let mut memory = FakeMemory::new(0x78);
    memory.put(0x28, 1u8);
    memory.put(0x50, 1u8);
    // SAFETY: the buffer is at least as large as the layout and lives for the borrow.
    let layout = unsafe { &*(memory.addr() as *const ClientWaypointLayout) };
    assert!(layout.color.get().is_some());
    assert!(layout.texture_path.get().is_some());
}

#[test]
fn actor_id_needs_the_entity_vtable() {
    let mut memory = FakeMemory::new(0x98);
    memory.put(0x00, 0xF13DFD0usize);
    memory.put(0x78, 4242i64);
    let addr = memory.addr();
    // SAFETY: the buffer is a 0x98 byte fake waypoint that lives for the call.
    unsafe {
        assert_eq!(ClientWaypointLayout::actor_id(addr, 0xF13DFD0), Some(4242));
        assert_eq!(ClientWaypointLayout::actor_id(addr, 0xF114660), None);
        assert_eq!(ClientWaypointLayout::actor_id(0, 0xF13DFD0), None);
    }
}
