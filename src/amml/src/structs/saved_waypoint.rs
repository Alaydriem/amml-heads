use heads_core::services::TagCodec;
use heads_core::structs::TagColor;

/// One tagged waypoint: where it is, the exact colour bytes it had, and the player it follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavedWaypoint {
    waypoint: usize,
    // the 20 bytes of the `optional<Color>`: four floats, the engaged flag and its padding
    color: [u8; Self::COLOR_BYTES],
    actor: i64,
    has_actor: bool,
}

impl SavedWaypoint {
    pub const COLOR_BYTES: usize = 20;
    const FLAG_INDEX: usize = 16;

    pub const EMPTY: Self = Self { waypoint: 0, color: [0; Self::COLOR_BYTES], actor: 0, has_actor: false };

    pub fn new(waypoint: usize, color: [u8; Self::COLOR_BYTES], actor: Option<i64>) -> Self {
        Self { waypoint, color, actor: actor.unwrap_or(0), has_actor: actor.is_some() }
    }

    pub fn waypoint(&self) -> usize {
        self.waypoint
    }

    pub fn color_bytes(&self) -> &[u8; Self::COLOR_BYTES] {
        &self.color
    }

    pub fn actor(&self) -> Option<i64> {
        self.has_actor.then_some(self.actor)
    }

    /// The colour the vanilla blit would have received: the saved colour packed, or white when it was empty.
    pub fn vanilla_packed(&self) -> u32 {
        if self.color[Self::FLAG_INDEX] == 0 {
            return 0xFFFF_FFFF;
        }
        let lane = |i: usize| {
            f32::from_le_bytes([self.color[i * 4], self.color[i * 4 + 1], self.color[i * 4 + 2], self.color[i * 4 + 3]])
        };
        TagCodec::pack(TagColor::new(lane(0), lane(1), lane(2), lane(3)))
    }
}
