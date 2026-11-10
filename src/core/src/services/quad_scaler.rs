use crate::structs::IconRect;

pub struct QuadScaler;

impl QuadScaler {
    const MIN_FACTOR: f32 = 1.0;
    const MAX_FACTOR: f32 = 2.0;

    /// Grows a rectangle about its centre. The factor is clamped to 1.0..=2.0.
    pub fn scale(rect: IconRect, factor: f32) -> IconRect {
        let factor = factor.clamp(Self::MIN_FACTOR, Self::MAX_FACTOR);
        let w = rect.w * factor;
        let h = rect.h * factor;
        IconRect::new(rect.x + (rect.w - w) / 2.0, rect.y + (rect.h - h) / 2.0, w, h)
    }
}
