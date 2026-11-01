use crate::errors::FaceError;
use crate::structs::{FaceImage, HeadUv, SkinView, UvRect};

pub struct FaceExtractor;

impl FaceExtractor {
    const RGBA8: u32 = 4;
    // how far a rectangle may reach past the image edge before it counts as outside
    const EDGE_TOLERANCE: f32 = 1e-3;

    /// Copies the face and hat regions of a classic skin into a 32x32 image, hat over face.
    pub fn extract(skin: &SkinView) -> Result<FaceImage, FaceError> {
        if skin.format != Self::RGBA8 {
            return Err(FaceError::UnsupportedFormat(skin.format));
        }
        let size_ok =
            matches!(skin.width, 64 | 128 | 256) && (skin.height == skin.width || skin.height * 2 == skin.width);
        if !size_ok {
            return Err(FaceError::UnsupportedSize { width: skin.width, height: skin.height });
        }
        // a 64x32 skin is a 64x32 texture, so its head rectangles scale only by the width
        let texture_height = if skin.height == skin.width { 64 } else { 32 };
        Self::extract_at(skin, &HeadUv { texture_height, ..HeadUv::classic() })
    }

    /// Copies the face and hat regions named by `head` into a 32x32 image, hat over face. The rectangles are in
    /// texture pixels and scale with the image. `EmptyFace` when no pixel of the face is visible.
    pub fn extract_at(skin: &SkinView, head: &HeadUv) -> Result<FaceImage, FaceError> {
        if skin.format != Self::RGBA8 {
            return Err(FaceError::UnsupportedFormat(skin.format));
        }
        let unsupported = FaceError::UnsupportedSize { width: skin.width, height: skin.height };
        if skin.width == 0 || skin.height == 0 || head.texture_width == 0 || head.texture_height == 0 {
            return Err(unsupported);
        }
        let need = skin.width as usize * skin.height as usize * 4;
        if skin.bytes.len() < need {
            return Err(FaceError::ShortBuffer { need, have: skin.bytes.len() });
        }

        let scale_x = skin.width as f32 / head.texture_width as f32;
        let scale_y = skin.height as f32 / head.texture_height as f32;
        let face = Self::pixel_rect(head.face, scale_x, scale_y, skin).ok_or(unsupported)?;
        let hat = head.hat.and_then(|rect| Self::pixel_rect(rect, scale_x, scale_y, skin));

        let mut pixels = [0u8; 4096];
        let mut visible = false;
        for y in 0..FaceImage::SIDE {
            for x in 0..FaceImage::SIDE {
                let under = Self::sample(skin, &face, x, y);
                visible |= under[3] != 0;
                let over = hat.map_or([0; 4], |hat| Self::sample(skin, &hat, x, y));
                let at = ((y * FaceImage::SIDE + x) * 4) as usize;
                pixels[at..at + 4].copy_from_slice(&Self::over(over, under));
            }
        }
        if !visible {
            return Err(FaceError::EmptyFace);
        }
        Ok(FaceImage::new(pixels))
    }

    // the rectangle in image pixels as [x, y, w, h]; None when it is empty, not finite or reaches outside the image
    fn pixel_rect(rect: UvRect, scale_x: f32, scale_y: f32, skin: &SkinView) -> Option<[f32; 4]> {
        let scaled = [rect.x * scale_x, rect.y * scale_y, rect.w * scale_x, rect.h * scale_y];
        let inside = scaled.iter().all(|value| value.is_finite())
            && scaled[0] >= 0.0
            && scaled[1] >= 0.0
            && scaled[2] > 0.0
            && scaled[3] > 0.0
            && scaled[0] + scaled[2] <= skin.width as f32 + Self::EDGE_TOLERANCE
            && scaled[1] + scaled[3] <= skin.height as f32 + Self::EDGE_TOLERANCE;
        inside.then_some(scaled)
    }

    // nearest pixel of output pixel (x, y) inside the rectangle, kept inside the image
    fn sample(skin: &SkinView, rect: &[f32; 4], x: u32, y: u32) -> [u8; 4] {
        let side = FaceImage::SIDE as f32;
        let source_x = (rect[0] + (x as f32 + 0.5) * rect[2] / side).floor().max(0.0) as u32;
        let source_y = (rect[1] + (y as f32 + 0.5) * rect[3] / side).floor().max(0.0) as u32;
        let at = ((source_y.min(skin.height - 1) * skin.width + source_x.min(skin.width - 1)) * 4) as usize;
        [skin.bytes[at], skin.bytes[at + 1], skin.bytes[at + 2], skin.bytes[at + 3]]
    }

    // Source-over in integer arithmetic, rounded to nearest.
    fn over(top: [u8; 4], bottom: [u8; 4]) -> [u8; 4] {
        let top_alpha = u32::from(top[3]);
        let bottom_weight = u32::from(bottom[3]) * (255 - top_alpha);
        let alpha_scaled = top_alpha * 255 + bottom_weight;
        if alpha_scaled == 0 {
            return [0; 4];
        }
        let mut out = [0u8; 4];
        for channel in 0..3 {
            let sum = u32::from(top[channel]) * top_alpha * 255 + u32::from(bottom[channel]) * bottom_weight;
            out[channel] = ((sum + alpha_scaled / 2) / alpha_scaled) as u8;
        }
        out[3] = alpha_scaled.div_ceil(255).min(255) as u8;
        out
    }
}
