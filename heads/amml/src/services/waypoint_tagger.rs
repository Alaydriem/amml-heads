#![allow(unsafe_code)]

use heads_core::services::TagCodec;

use super::MemoryReader;
use crate::layouts::ClientWaypointLayout;
use crate::structs::{SavedWaypoint, TagFrame, WaypointOffsets};

/// Writes a face index into the colour of each visible waypoint, and puts the colours back.
pub struct WaypointTagger;

impl WaypointTagger {
    // most nodes the walk visits; guards against a corrupt list
    const MAX_NODES: usize = 1024;

    /// Tags every visible waypoint in the list. Returns false, with nothing changed, when the list is null, longer
    /// than the walk cap, or holds more visible waypoints than the frame can tag.
    ///
    /// # Safety
    /// `list_head` must be null or the sentinel node of a live, circular `ClientLocatorBar` waypoint list, and the
    /// waypoints must stay alive and unchanged until `restore`, and `offsets` must be the layout of this build. This
    /// runs on the render thread, where the packet handler that changes the list cannot run.
    pub unsafe fn tag(list_head: usize, entity_vtable: usize, offsets: &WaypointOffsets, frame: &mut TagFrame) -> bool {
        frame.reset();
        if list_head == 0 {
            return false;
        }
        // SAFETY: the caller guarantees the list is live.
        if !unsafe { Self::collect(list_head, entity_vtable, offsets, frame) } {
            frame.reset();
            return false;
        }
        for (index, saved) in frame.saved().iter().enumerate() {
            let tag = TagCodec::encode(index as u16);
            let value = [tag.r, tag.g, tag.b, tag.a];
            // SAFETY: each saved waypoint was read live in `collect`; the colour and its engaged byte are inside it.
            unsafe {
                ((saved.waypoint() + offsets.color) as *mut [f32; 4]).write_unaligned(value);
                ((saved.waypoint() + offsets.color_engaged) as *mut u8).write(1);
            }
        }
        frame.activate();
        true
    }

    /// Puts the saved colour bytes back and empties the frame.
    ///
    /// # Safety
    /// The waypoints tagged by `tag` must still be alive, and `offsets` must be the ones `tag` used.
    pub unsafe fn restore(frame: &mut TagFrame, offsets: &WaypointOffsets) {
        for saved in frame.saved() {
            // SAFETY: the caller guarantees the waypoint is alive; the saved colour bytes are inside it.
            unsafe {
                ((saved.waypoint() + offsets.color) as *mut [u8; SavedWaypoint::COLOR_BYTES])
                    .write_unaligned(*saved.color_bytes())
            };
        }
        frame.reset();
    }

    // Fills the frame with the visible waypoints. False when the list is corrupt or does not fit the frame.
    unsafe fn collect(list_head: usize, entity_vtable: usize, offsets: &WaypointOffsets, frame: &mut TagFrame) -> bool {
        // SAFETY: the list head is a live node.
        let Some(mut node) = (unsafe { MemoryReader::read::<usize>(list_head) }) else { return false };
        let mut walked = 0;
        while node != list_head {
            walked += 1;
            if walked > Self::MAX_NODES || node == 0 {
                return false;
            }
            // SAFETY: `node` is a live list node.
            let (Some(next), Some(waypoint)) = (unsafe { MemoryReader::read::<usize>(node) }, unsafe {
                MemoryReader::read::<usize>(node + offsets.node_waypoint)
            }) else {
                return false;
            };
            if waypoint != 0 {
                // SAFETY: the node holds a live waypoint, and the visible flag is inside it.
                let visible = unsafe { MemoryReader::read::<u8>(waypoint + offsets.visible) };
                if visible.is_some_and(|visible| visible != 0) {
                    // SAFETY: as above; the saved colour bytes are inside the waypoint.
                    let Some(color) =
                        (unsafe { MemoryReader::read::<[u8; SavedWaypoint::COLOR_BYTES]>(waypoint + offsets.color) })
                    else {
                        return false;
                    };
                    // SAFETY: the waypoint is live; the vtable check guards the actor id read.
                    let actor = unsafe { ClientWaypointLayout::actor_id(waypoint, entity_vtable, offsets.actor_id) };
                    if !frame.push(SavedWaypoint::new(waypoint, color, actor)) {
                        return false;
                    }
                }
            }
            node = next;
        }
        true
    }
}
