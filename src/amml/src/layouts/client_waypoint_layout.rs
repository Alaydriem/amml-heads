#![allow(unsafe_code)]

use amml_msvc::{MsvcString, Optional};

use crate::services::MemoryReader;

/// Mirror of the 26.60 `ClientWaypoint` (0x78 bytes). `ClientEntityWaypoint` adds the actor id after it.
#[repr(C)]
pub struct ClientWaypointLayout {
    pub vtable: usize,
    pub position: [f32; 3],
    pub dimension: u32,
    pub color: Optional<[f32; 4]>,
    pub texture_path: Optional<MsvcString>,
    pub icon_size: [f32; 2],
    pub visible: bool,
    pub client_position_authority: bool,
    pub reserved: [u8; 6],
    pub smoothed_position: Optional<[f32; 3]>,
}

impl ClientWaypointLayout {
    /// The actor id of a `ClientEntityWaypoint`, read at `actor_offset`, or `None` for a null pointer or another
    /// waypoint type.
    ///
    /// # Safety
    /// `waypoint` must be null or point at a live `ClientWaypoint`, and at a live `ClientEntityWaypoint` that holds
    /// an `ActorUniqueID` at `actor_offset` when its vtable is `entity_vtable`.
    pub unsafe fn actor_id(waypoint: usize, entity_vtable: usize, actor_offset: usize) -> Option<i64> {
        // SAFETY: the caller guarantees the waypoint is live; the reader checks for null.
        let vtable: usize = unsafe { MemoryReader::read(waypoint)? };
        if vtable != entity_vtable {
            return None;
        }
        // SAFETY: the vtable matched, so the object is a ClientEntityWaypoint with the actor id at `actor_offset`.
        unsafe { MemoryReader::read(waypoint + actor_offset) }
    }
}
