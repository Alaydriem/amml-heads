use crate::structs::TagColor;

pub struct TagCodec;

impl TagCodec {
    const MARKER: u8 = 0xA5;

    /// The colour that marks a waypoint with a face index.
    pub fn encode(index: u16) -> TagColor {
        TagColor::new(Self::lane(Self::MARKER), Self::lane((index >> 8) as u8), Self::lane((index & 0xFF) as u8), 1.0)
    }

    /// The face index in a packed colour, or `None` when the colour is not a tag.
    pub fn decode(packed: u32) -> Option<u16> {
        let alpha = (packed >> 24) as u8;
        let red = (packed >> 16) as u8;
        if alpha != 0xFF || red != Self::MARKER {
            return None;
        }
        Some((packed & 0xFFFF) as u16)
    }

    /// The vanilla 0xAARRGGBB packing: clamp each lane to 0..=1, scale by 255, truncate.
    pub fn pack(color: TagColor) -> u32 {
        (Self::byte(color.a) << 24) | (Self::byte(color.r) << 16) | (Self::byte(color.g) << 8) | Self::byte(color.b)
    }

    // A float that packs back to exactly `value` under truncation.
    fn lane(value: u8) -> f32 {
        let exact = f32::from(value) / 255.0;
        if Self::byte(exact) == u32::from(value) { exact } else { (f32::from(value) + 0.5) / 255.0 }
    }

    fn byte(lane: f32) -> u32 {
        (lane.clamp(0.0, 1.0) * 255.0) as u32
    }
}
