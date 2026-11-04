use super::SavedWaypoint;

/// The layout of the `ClientLocatorBar` waypoint list and of a `ClientWaypoint`, read from the field entries when
/// the mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaypointOffsets {
    /// The list head pointer of the waypoint map inside `ClientLocatorBar`.
    pub list_head: usize,
    /// The `unique_ptr<ClientWaypoint>` inside one list node.
    pub node_waypoint: usize,
    /// `mColor` (a `std::optional<mce::Color>`) inside `ClientWaypoint`.
    pub color: usize,
    /// The engaged byte of `mColor`.
    pub color_engaged: usize,
    /// The visible flag inside `ClientWaypoint`.
    pub visible: usize,
    /// The `ActorUniqueID` inside `ClientEntityWaypoint`.
    pub actor_id: usize,
}

impl WaypointOffsets {
    /// `None` when the engaged byte is not inside the `SavedWaypoint::COLOR_BYTES` saved from the colour, because
    /// a restore would then leave the tag in the game.
    pub fn new(
        list_head: usize,
        node_waypoint: usize,
        color: usize,
        color_engaged: usize,
        visible: usize,
        actor_id: usize,
    ) -> Option<Self> {
        let saved = color.checked_add(SavedWaypoint::COLOR_BYTES)?;
        if color_engaged < color || color_engaged >= saved {
            return None;
        }
        Some(Self { list_head, node_waypoint, color, color_engaged, visible, actor_id })
    }
}
