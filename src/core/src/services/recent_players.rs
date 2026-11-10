use std::collections::VecDeque;

/// The identity (UUID, XUID, name) of players seen in a player list that hold no glyph cell. Name and XUID lookups
/// count these players too, so a name or XUID shared with one of them is ambiguous and gives no head.
///
/// Bound: at most `CAPACITY` (240) players. Noting a new player when full evicts the oldest one noted. Noting a
/// known player again updates it and makes it the newest.
pub struct RecentPlayers {
    entries: VecDeque<([u8; 16], String, String)>,
}

impl RecentPlayers {
    pub const CAPACITY: usize = 240;

    pub fn new() -> Self {
        Self { entries: VecDeque::with_capacity(Self::CAPACITY) }
    }

    pub fn note(&mut self, uuid: [u8; 16], xuid: &str, name: &str) {
        let reused = match self.entries.iter().position(|entry| entry.0 == uuid) {
            Some(at) => self.entries.remove(at),
            None if self.entries.len() >= Self::CAPACITY => self.entries.pop_front(),
            None => None,
        };
        let (_, mut kept_xuid, mut kept_name) = reused.unwrap_or_default();
        kept_xuid.clear();
        kept_xuid.push_str(xuid);
        kept_name.clear();
        kept_name.push_str(name);
        self.entries.push_back((uuid, kept_xuid, kept_name));
    }

    pub fn remove(&mut self, uuid: [u8; 16]) {
        self.entries.retain(|entry| entry.0 != uuid);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Each player as (UUID, XUID, name), oldest first.
    pub fn iter(&self) -> impl Iterator<Item = ([u8; 16], &str, &str)> {
        self.entries.iter().map(|entry| (entry.0, entry.1.as_str(), entry.2.as_str()))
    }
}

impl Default for RecentPlayers {
    fn default() -> Self {
        Self::new()
    }
}
