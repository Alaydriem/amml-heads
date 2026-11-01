#![allow(unsafe_code)]

use amml_msvc::{MsvcString, Optional};

use crate::services::MemoryReader;

/// Mirror of the 26.60 `ClientWaypoint` (0x78 bytes). `ClientEntityWaypoint` adds the actor id at 0x78.
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
    pub const ACTOR_ID_OFFSET: usize = 0x78;

    /// The actor id of a `ClientEntityWaypoint`, or `None` for a null pointer or another waypoint type.
    ///
    /// # Safety
    /// `waypoint` must be null or point at a live `ClientWaypoint` of at least 0x78 bytes, and at 0x98 bytes when
    /// its vtable is `entity_vtable`.
    pub unsafe fn actor_id(waypoint: usize, entity_vtable: usize) -> Option<i64> {
        // SAFETY: the caller guarantees the waypoint is live; the reader checks for null.
        let vtable: usize = unsafe { MemoryReader::read(waypoint)? };
        if vtable != entity_vtable {
            return None;
        }
        // SAFETY: the vtable matched, so the object is a ClientEntityWaypoint of 0x98 bytes.
        unsafe { MemoryReader::read(waypoint + Self::ACTOR_ID_OFFSET) }
    }
}
