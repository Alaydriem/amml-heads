use heads_core::errors::FaceError;
use heads_core::services::FaceExtractor;
use heads_core::structs::{FaceImage, HeadUv, SkinView, UvRect};

struct SkinBuilder {
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

impl SkinBuilder {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height, bytes: vec![0; (width * height * 4) as usize] }
    }

    fn fill(mut self, x0: u32, y0: u32, side: u32, rgba: [u8; 4]) -> Self {
        for y in y0..y0 + side {
            for x in x0..x0 + side {
                self.set(x, y, rgba);
            }
        }
        self
    }

    fn set(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        let at = ((y * self.width + x) * 4) as usize;
        self.bytes[at..at + 4].copy_from_slice(&rgba);
    }

    fn view(&self) -> SkinView<'_> {
        SkinView::new(self.width, self.height, 4, &self.bytes)
    }
}

const RED: [u8; 4] = [255, 0, 0, 255];
const BLUE: [u8; 4] = [0, 0, 255, 255];

fn assert_all(face: &FaceImage, expected: [u8; 4]) {
    for y in 0..FaceImage::SIDE {
        for x in 0..FaceImage::SIDE {
            assert_eq!(face.pixel(x, y), expected, "pixel {x},{y}");
        }
    }
}

#[test]
fn face_without_hat_upscales_nearest() {
    let skin = SkinBuilder::new(64, 64).fill(8, 8, 8, RED);
    let face = FaceExtractor::extract(&skin.view()).unwrap();
    assert_all(&face, RED);

    let mut skin = SkinBuilder::new(64, 64).fill(8, 8, 8, RED);
    skin.set(8, 8, [1, 2, 3, 255]);
    skin.set(9, 8, [4, 5, 6, 255]);
    let face = FaceExtractor::extract(&skin.view()).unwrap();
    assert_eq!(face.pixel(0, 0), [1, 2, 3, 255]);
    assert_eq!(face.pixel(3, 3), [1, 2, 3, 255]);
    assert_eq!(face.pixel(4, 0), [4, 5, 6, 255]);
}

#[test]
fn opaque_hat_covers_face() {
    let skin = SkinBuilder::new(64, 64).fill(8, 8, 8, RED).fill(40, 8, 8, BLUE);
    let face = FaceExtractor::extract(&skin.view()).unwrap();
    assert_all(&face, BLUE);
}

#[test]
fn half_alpha_hat_blends() {
    let skin = SkinBuilder::new(64, 64).fill(8, 8, 8, [200, 0, 0, 255]).fill(40, 8, 8, [0, 0, 200, 128]);
    let face = FaceExtractor::extract(&skin.view()).unwrap();
    let got = face.pixel(5, 5);
    for (channel, want) in [100i32, 0, 100, 255].into_iter().enumerate() {
        assert!((i32::from(got[channel]) - want).abs() <= 1, "channel {channel}: {got:?}");
    }
}

#[test]
fn accepts_64x32() {
    let skin = SkinBuilder::new(64, 32).fill(8, 8, 8, RED);
    let face = FaceExtractor::extract(&skin.view()).unwrap();
    assert_all(&face, RED);
}

#[test]
fn scales_128() {
    let mut skin = SkinBuilder::new(128, 128).fill(16, 16, 16, RED);
    skin.set(16, 16, [1, 2, 3, 255]);
    let face = FaceExtractor::extract(&skin.view()).unwrap();
    assert_eq!(face.pixel(0, 0), [1, 2, 3, 255]);
    assert_eq!(face.pixel(31, 31), RED);
}

#[test]
fn rejects_rgb8() {
    let skin = SkinBuilder::new(64, 64);
    let view = SkinView::new(64, 64, 3, &skin.bytes);
    assert_eq!(FaceExtractor::extract(&view).unwrap_err(), FaceError::UnsupportedFormat(3));
}

#[test]
fn rejects_width_96() {
    let bytes = vec![0u8; 96 * 96 * 4];
    let view = SkinView::new(96, 96, 4, &bytes);
    assert_eq!(FaceExtractor::extract(&view).unwrap_err(), FaceError::UnsupportedSize { width: 96, height: 96 });
}

#[test]
fn rejects_short_buffer() {
    let bytes = vec![0u8; 100];
    let view = SkinView::new(64, 64, 4, &bytes);
    assert_eq!(FaceExtractor::extract(&view).unwrap_err(), FaceError::ShortBuffer { need: 16384, have: 100 });
}

fn classic_head() -> HeadUv {
    HeadUv::classic()
}

#[test]
fn fully_transparent_face_is_empty() {
    let skin = SkinBuilder::new(64, 64).fill(40, 8, 8, BLUE);

    assert_eq!(FaceExtractor::extract(&skin.view()).unwrap_err(), FaceError::EmptyFace);
    assert_eq!(FaceExtractor::extract_at(&skin.view(), &classic_head()).unwrap_err(), FaceError::EmptyFace);
}

#[test]
fn a_texture_width_of_64_on_a_128_image_scales_by_two() {
    let mut skin = SkinBuilder::new(128, 128).fill(16, 16, 16, RED);
    skin.set(16, 16, [1, 2, 3, 255]);

    let face = FaceExtractor::extract_at(&skin.view(), &classic_head()).unwrap();

    assert_eq!(face.pixel(0, 0), [1, 2, 3, 255]);
    assert_eq!(face.pixel(31, 31), RED);
}

#[test]
fn a_persona_atlas_is_read_where_the_geometry_puts_the_head() {
    // the classic head region is empty; the head is at (96,128) in a 256 atlas
    let mut skin = SkinBuilder::new(256, 256).fill(96, 128, 8, RED);
    skin.set(96, 128, [1, 2, 3, 255]);
    skin.set(97, 128, [4, 5, 6, 255]);
    let head = HeadUv::new(UvRect::new(96.0, 128.0, 8.0, 8.0), None, 256, 256);

    let face = FaceExtractor::extract_at(&skin.view(), &head).unwrap();

    assert_eq!(face.pixel(0, 0), [1, 2, 3, 255]);
    assert_eq!(face.pixel(3, 3), [1, 2, 3, 255]);
    assert_eq!(face.pixel(4, 0), [4, 5, 6, 255]);
    assert_eq!(face.pixel(31, 31), RED);
    assert_eq!(FaceExtractor::extract(&skin.view()).unwrap_err(), FaceError::EmptyFace);
}

#[test]
fn the_hat_from_the_geometry_is_blended() {
    let skin = SkinBuilder::new(256, 256).fill(96, 128, 8, RED).fill(160, 128, 8, BLUE);
    let head = HeadUv::new(UvRect::new(96.0, 128.0, 8.0, 8.0), Some(UvRect::new(160.0, 128.0, 8.0, 8.0)), 256, 256);

    let face = FaceExtractor::extract_at(&skin.view(), &head).unwrap();

    assert_eq!(face.pixel(10, 10), BLUE);
}

#[test]
fn a_face_rect_outside_the_image_is_an_unsupported_size() {
    let skin = SkinBuilder::new(64, 64).fill(8, 8, 8, RED);
    let outside = HeadUv::new(UvRect::new(60.0, 8.0, 8.0, 8.0), None, 64, 64);
    let empty = HeadUv::new(UvRect::new(8.0, 8.0, 0.0, 8.0), None, 64, 64);
    let no_texture = HeadUv::new(UvRect::new(8.0, 8.0, 8.0, 8.0), None, 0, 64);

    for head in [outside, empty, no_texture] {
        assert_eq!(
            FaceExtractor::extract_at(&skin.view(), &head).unwrap_err(),
            FaceError::UnsupportedSize { width: 64, height: 64 }
        );
    }
}

#[test]
fn a_hat_rect_outside_the_image_is_ignored() {
    let skin = SkinBuilder::new(64, 64).fill(8, 8, 8, RED);
    let head = HeadUv::new(UvRect::new(8.0, 8.0, 8.0, 8.0), Some(UvRect::new(60.0, 8.0, 8.0, 8.0)), 64, 64);

    assert_all(&FaceExtractor::extract_at(&skin.view(), &head).unwrap(), RED);
}

#[test]
fn extract_at_checks_the_format_and_the_buffer() {
    let bytes = vec![0u8; 100];
    let rgb = SkinView::new(64, 64, 3, &bytes);
    let short = SkinView::new(64, 64, 4, &bytes);

    assert_eq!(FaceExtractor::extract_at(&rgb, &classic_head()).unwrap_err(), FaceError::UnsupportedFormat(3));
    assert_eq!(
        FaceExtractor::extract_at(&short, &classic_head()).unwrap_err(),
        FaceError::ShortBuffer { need: 16384, have: 100 }
    );
}

#[test]
fn an_animated_face_puts_the_hat_over_the_face_from_the_first_frame() {
    let mut skin = SkinBuilder::new(32, 64).fill(8, 8, 8, RED).fill(8, 40, 8, BLUE);
    // the hat covers only the top row of the face, as hair does
    for x in 8..16 {
        skin.set(x, 24, BLUE);
    }

    let face = FaceExtractor::extract_at(&skin.view(), &HeadUv::animated_face()).unwrap();

    assert_eq!(face.pixel(10, 1), BLUE);
    assert_eq!(face.pixel(10, 20), RED);
}
