use heads_core::services::IconSampler;
use heads_core::structs::{FaceImage, IconImage};

fn face() -> FaceImage {
    let mut pixels = [0u8; 4096];
    for y in 0..32u32 {
        for x in 0..32u32 {
            let at = ((y * 32 + x) * 4) as usize;
            pixels[at] = x as u8;
            pixels[at + 1] = y as u8;
            pixels[at + 2] = 7;
            pixels[at + 3] = 255;
        }
    }
    FaceImage::new(pixels)
}

#[test]
fn a_face_lands_at_10x10_on_the_icon_baseline_of_the_cell() {
    let face = face();

    let icon = IconSampler::sample(&face);

    for y in 0..16 {
        for x in 0..16 {
            let expected =
                if x < 10 && (2..12).contains(&y) { face.pixel(x * 32 / 10, (y - 2) * 32 / 10) } else { [0, 0, 0, 0] };
            assert_eq!(icon.pixel(x, y), expected, "at {x},{y}");
        }
    }
    assert_eq!(icon.pixels.len(), (IconImage::SIDE * IconImage::SIDE * 4) as usize);
}
