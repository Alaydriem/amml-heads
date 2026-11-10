use super::SavedWaypoint;

/// The waypoints tagged in the current frame. Fixed size: it holds at most `CAPACITY` waypoints.
pub struct TagFrame {
    saved: [SavedWaypoint; Self::CAPACITY],
    count: usize,
    active: bool,
}

impl TagFrame {
    pub const CAPACITY: usize = 256;

    pub fn new() -> Self {
        Self { saved: [SavedWaypoint::EMPTY; Self::CAPACITY], count: 0, active: false }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Whether the waypoint colours are tagged right now.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// The waypoint saved under a tag index.
    pub fn get(&self, index: usize) -> Option<SavedWaypoint> {
        (index < self.count).then(|| self.saved[index])
    }

    pub(crate) fn saved(&self) -> &[SavedWaypoint] {
        &self.saved[..self.count]
    }

    /// Adds a waypoint. Returns false when the frame is full.
    pub(crate) fn push(&mut self, waypoint: SavedWaypoint) -> bool {
        if self.count == Self::CAPACITY {
            return false;
        }
        self.saved[self.count] = waypoint;
        self.count += 1;
        true
    }

    pub(crate) fn activate(&mut self) {
        self.active = true;
    }

    pub(crate) fn reset(&mut self) {
        self.count = 0;
        self.active = false;
    }
}

impl Default for TagFrame {
    fn default() -> Self {
        Self::new()
    }
}
