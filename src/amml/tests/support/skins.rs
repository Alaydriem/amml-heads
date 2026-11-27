use amml_api::AnimatedKind;
use amml_api::testing::FakeSkin;

// the skins the tests give to the fake game; a face region is painted in one opaque colour
pub struct Skins;

impl Skins {
    pub const CLASSIC_112: &'static str = r#"{
      "format_version": "1.12.0",
      "minecraft:geometry": [
        {
          "description": { "identifier": "geometry.humanoid.custom", "texture_width": 64, "texture_height": 64 },
          "bones": [
            { "name": "Head", "cubes": [{ "origin": [-4, 24, -4], "size": [8, 8, 8], "uv": [32, 0] }] }
          ]
        }
      ]
    }"#;

    // a 64x64 skin with the classic face at (8, 8) painted `rgb`
    pub fn classic(rgb: [u8; 3]) -> FakeSkin {
        FakeSkin { image: Some((64, 64, Self::paint(64, 64, [(8, 8, 8, 8, rgb)]))), ..FakeSkin::default() }
    }

    // a 64x64 skin whose geometry puts the head at uv (32, 0): the face is at (40, 8), the classic place is red
    pub fn geometry(rgb: [u8; 3]) -> FakeSkin {
        FakeSkin {
            image: Some((64, 64, Self::paint(64, 64, [(8, 8, 8, 8, [255, 0, 0]), (40, 8, 8, 8, rgb)]))),
            geometry_name: Some("geometry.humanoid.custom".to_string()),
            geometry_json: Some(Self::CLASSIC_112.to_string()),
            ..FakeSkin::default()
        }
    }

    // a persona skin: a red classic face, and the animated face image painted `rgb` at (8, 8) of a 32x64 texture
    pub fn persona(rgb: [u8; 3]) -> FakeSkin {
        FakeSkin {
            image: Some((64, 64, Self::paint(64, 64, [(8, 8, 8, 8, [255, 0, 0])]))),
            animated: vec![(AnimatedKind::Face, (32, 64, Self::paint(32, 64, [(8, 8, 8, 8, rgb)])))],
            persona: true,
            ..FakeSkin::default()
        }
    }

    // a skin whose blob is too short for its size: the loader gives no image
    pub fn unreadable() -> FakeSkin {
        FakeSkin { image: Some((64, 64, vec![255; 16])), ..FakeSkin::default() }
    }

    // a skin that is all transparent: no face is visible
    pub fn empty() -> FakeSkin {
        FakeSkin { image: Some((64, 64, vec![0; 64 * 64 * 4])), ..FakeSkin::default() }
    }

    fn paint<const N: usize>(width: u32, height: u32, regions: [(u32, u32, u32, u32, [u8; 3]); N]) -> Vec<u8> {
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        for (x0, y0, w, h, rgb) in regions {
            for y in y0..y0 + h {
                for x in x0..x0 + w {
                    let at = ((y * width + x) * 4) as usize;
                    pixels[at..at + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
                }
            }
        }
        pixels
    }
}
