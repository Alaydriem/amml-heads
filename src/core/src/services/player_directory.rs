use crate::services::RecentPlayers;

/// Maps XUIDs and names to player UUIDs. A player that holds a glyph cell is recorded; a player seen without one is
/// noted in a `RecentPlayers`. A UUID is in one of the two, never both. A lookup counts both, and finds a player
/// only when exactly one UUID matches.
///
/// Bound: at most `CAPACITY` (240) recorded players for the life of the process, the same as the glyph cells.
/// There is no eviction and a Remove never deletes an entry, so a chat line that names a departed player still
/// finds the face. When full, `record` refuses a new UUID. The noted players have the bound of `RecentPlayers`.
pub struct PlayerDirectory {
    entries: Vec<([u8; 16], String, String)>,
    recent: RecentPlayers,
}

impl PlayerDirectory {
    pub const CAPACITY: usize = 240;

    pub fn new() -> Self {
        Self { entries: Vec::with_capacity(Self::CAPACITY), recent: RecentPlayers::new() }
    }

    pub fn record(&mut self, uuid: [u8; 16], xuid: &str, name: &str) -> bool {
        if self.update(uuid, xuid, name) {
            return true;
        }
        if self.entries.len() >= Self::CAPACITY {
            return false;
        }
        self.recent.remove(uuid);
        self.entries.push((uuid, xuid.to_owned(), name.to_owned()));
        true
    }

    /// Keeps the identity of a player seen in a player list, whether or not it gets a cell.
    pub fn note(&mut self, uuid: [u8; 16], xuid: &str, name: &str) {
        if !self.update(uuid, xuid, name) {
            self.recent.note(uuid, xuid, name);
        }
    }

    pub fn by_xuid(&self, xuid: &str) -> Option<[u8; 16]> {
        if xuid.is_empty() {
            return None;
        }
        self.sole(|entry_xuid, _| entry_xuid == xuid)
    }

    pub fn by_name(&self, name: &str) -> Option<[u8; 16]> {
        self.sole(|_, entry_name| entry_name == name)
    }

    pub fn names(&self) -> impl Iterator<Item = (&str, [u8; 16])> {
        self.all().map(|entry| (entry.2, entry.0))
    }

    fn update(&mut self, uuid: [u8; 16], xuid: &str, name: &str) -> bool {
        let Some(entry) = self.entries.iter_mut().find(|entry| entry.0 == uuid) else {
            return false;
        };
        entry.1.clear();
        entry.1.push_str(xuid);
        entry.2.clear();
        entry.2.push_str(name);
        true
    }

    fn all(&self) -> impl Iterator<Item = ([u8; 16], &str, &str)> {
        self.entries.iter().map(|entry| (entry.0, entry.1.as_str(), entry.2.as_str())).chain(self.recent.iter())
    }

    // the UUID of the one player `matches` accepts, given (XUID, name)
    fn sole(&self, matches: impl Fn(&str, &str) -> bool) -> Option<[u8; 16]> {
        let mut found = self.all().filter(|entry| matches(entry.1, entry.2));
        let first = found.next()?;
        if found.next().is_some() {
            return None;
        }
        Some(first.0)
    }
}

impl Default for PlayerDirectory {
    fn default() -> Self {
        Self::new()
    }
}
