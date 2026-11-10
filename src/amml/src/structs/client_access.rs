use super::WaypointOffsets;

/// What the render side needs from `ClientInstance` and `LocalPlayer`, resolved from the recipe entries at enable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientAccess {
    /// The vtable slot of `LocalPlayer* ClientInstance::getLocalPlayer()`.
    pub get_local_player_slot: usize,
    /// The vtable slot of `shared_ptr<TextureGroup> ClientInstance::getTextureGroup()`, which returns through a
    /// hidden return pointer.
    pub get_texture_group_slot: usize,
    /// `mClientLocatorBar` inside `LocalPlayer`.
    pub locator_bar_offset: usize,
    /// The `ClientEntityWaypoint` vtable address.
    pub entity_vtable: usize,
    /// The waypoint list and waypoint layout.
    pub waypoints: WaypointOffsets,
}

impl ClientAccess {
    pub fn new(
        get_local_player_slot: usize,
        get_texture_group_slot: usize,
        locator_bar_offset: usize,
        entity_vtable: usize,
        waypoints: WaypointOffsets,
    ) -> Self {
        Self { get_local_player_slot, get_texture_group_slot, locator_bar_offset, entity_vtable, waypoints }
    }
}
