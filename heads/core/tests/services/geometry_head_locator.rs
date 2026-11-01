use heads_core::services::GeometryHeadLocator;
use heads_core::structs::{HeadUv, UvRect};

const CLASSIC_112: &str = r#"{
  "format_version": "1.12.0",
  "minecraft:geometry": [
    {
      "description": { "identifier": "geometry.humanoid.custom", "texture_width": 64, "texture_height": 64 },
      "bones": [
        { "name": "body", "cubes": [{ "origin": [-4, 12, -2], "size": [8, 12, 4], "uv": [16, 16] }] },
        { "name": "Head", "cubes": [{ "origin": [-4, 24, -4], "size": [8, 8, 8], "uv": [0, 0] }] },
        { "name": "hat", "cubes": [{ "origin": [-4, 24, -4], "size": [8, 8, 8], "uv": [32, 0], "inflate": 0.5 }] }
      ]
    }
  ]
}"#;

fn rect(x: f32, y: f32, w: f32, h: f32) -> UvRect {
    UvRect::new(x, y, w, h)
}

#[test]
fn classic_humanoid_1_12_gives_the_classic_head() {
    let head = GeometryHeadLocator::locate(CLASSIC_112, "geometry.humanoid.custom").unwrap();

    assert_eq!(head, HeadUv::new(rect(8.0, 8.0, 8.0, 8.0), Some(rect(40.0, 8.0, 8.0, 8.0)), 64, 64));
}

#[test]
fn the_1_8_format_matches_the_key_before_the_colon() {
    let json = r#"{
      "format_version": "1.8.0",
      "geometry.other": { "texturewidth": 32, "textureheight": 32, "bones": [] },
      "geometry.humanoid.custom:geometry.humanoid": {
        "texturewidth": 128,
        "textureheight": 64,
        "bones": [
          { "name": "head", "cubes": [{ "origin": [-4, 24, -4], "size": [8, 8, 8], "uv": [0, 0] }] },
          { "name": "hat", "cubes": [{ "origin": [-4, 24, -4], "size": [8, 8, 8], "uv": [32, 0] }] }
        ]
      }
    }"#;

    let head = GeometryHeadLocator::locate(json, "geometry.humanoid.custom").unwrap();

    assert_eq!(head.face, rect(8.0, 8.0, 8.0, 8.0));
    assert_eq!(head.hat, Some(rect(40.0, 8.0, 8.0, 8.0)));
    assert_eq!((head.texture_width, head.texture_height), (128, 64));
}

#[test]
fn a_per_face_uv_object_uses_the_north_face() {
    let json = r#"{
      "minecraft:geometry": [{
        "description": { "identifier": "geometry.persona_x", "texture_width": 256, "texture_height": 256 },
        "bones": [{ "name": "head", "cubes": [{
          "origin": [-4, 24, -4], "size": [8, 8, 8],
          "uv": {
            "north": { "uv": [96, 128], "uv_size": [8, 8] },
            "south": { "uv": [112, 128], "uv_size": [8, 8] }
          }
        }] }]
      }]
    }"#;

    let head = GeometryHeadLocator::locate(json, "geometry.persona_x").unwrap();

    assert_eq!(head.face, rect(96.0, 128.0, 8.0, 8.0));
    assert_eq!(head.hat, None);
    assert_eq!((head.texture_width, head.texture_height), (256, 256));
}

#[test]
fn the_identifier_picks_the_geometry() {
    let json = r#"{
      "minecraft:geometry": [
        { "description": { "identifier": "geometry.first", "texture_width": 64, "texture_height": 64 },
          "bones": [{ "name": "head", "cubes": [{ "size": [8, 8, 8], "uv": [0, 0] }] }] },
        { "description": { "identifier": "geometry.second", "texture_width": 64, "texture_height": 64 },
          "bones": [{ "name": "head", "cubes": [{ "size": [8, 8, 8], "uv": [16, 24] }] }] }
      ]
    }"#;

    assert_eq!(GeometryHeadLocator::locate(json, "geometry.second").unwrap().face, rect(24.0, 32.0, 8.0, 8.0));
    assert_eq!(GeometryHeadLocator::locate(json, "geometry.first").unwrap().face, rect(8.0, 8.0, 8.0, 8.0));
    assert_eq!(GeometryHeadLocator::locate(json, "geometry.unknown").unwrap().face, rect(8.0, 8.0, 8.0, 8.0));
}

#[test]
fn a_missing_texture_size_defaults_to_64() {
    let json = r#"{ "minecraft:geometry": [{
      "description": { "identifier": "geometry.x" },
      "bones": [{ "name": "head", "cubes": [{ "size": [8, 8, 8], "uv": [0, 0] }] }]
    }] }"#;

    let head = GeometryHeadLocator::locate(json, "geometry.x").unwrap();

    assert_eq!((head.texture_width, head.texture_height), (64, 64));
}

#[test]
fn no_head_bone_gives_none() {
    let json = r#"{ "minecraft:geometry": [{
      "description": { "identifier": "geometry.x", "texture_width": 64, "texture_height": 64 },
      "bones": [{ "name": "body", "cubes": [{ "size": [8, 8, 8], "uv": [0, 0] }] }]
    }] }"#;

    assert_eq!(GeometryHeadLocator::locate(json, "geometry.x"), None);
}

#[test]
fn a_head_bone_without_a_sized_cube_gives_none() {
    let json = r#"{ "minecraft:geometry": [{
      "description": { "identifier": "geometry.x" },
      "bones": [{ "name": "head", "cubes": [{ "origin": [0, 0, 0], "uv": [0, 0] }] }]
    }] }"#;

    assert_eq!(GeometryHeadLocator::locate(json, "geometry.x"), None);
}

#[test]
fn malformed_json_gives_none() {
    assert_eq!(GeometryHeadLocator::locate("{ \"minecraft:geometry\": [", "geometry.x"), None);
    assert_eq!(GeometryHeadLocator::locate("[1, 2, 3]", "geometry.x"), None);
    assert_eq!(GeometryHeadLocator::locate("", "geometry.x"), None);
}

#[test]
fn json_over_one_mebibyte_gives_none() {
    let padding = "x".repeat(1024 * 1024);
    let json = CLASSIC_112.replacen("1.12.0", &padding, 1);

    assert!(json.len() > 1024 * 1024);
    assert_eq!(GeometryHeadLocator::locate(&json, "geometry.humanoid.custom"), None);
}

#[test]
fn a_negative_value_gives_none() {
    let size = CLASSIC_112.replacen("\"size\": [8, 8, 8], \"uv\": [0, 0]", "\"size\": [-8, 8, 8], \"uv\": [0, 0]", 1);
    let uv = CLASSIC_112.replacen("\"uv\": [0, 0]", "\"uv\": [-1, 0]", 1);
    let texture = CLASSIC_112.replacen("\"texture_width\": 64", "\"texture_width\": -64", 1);

    for json in [size, uv, texture] {
        assert_eq!(GeometryHeadLocator::locate(&json, "geometry.humanoid.custom"), None);
    }
}

// a persona geometry with the animated face mesh after it; each poly lists [position, normal, uv] per vertex
const ANIMATED_FACE: &str = r#"{
  "minecraft:geometry": [
    {
      "description": { "identifier": "geometry.persona_x", "texture_width": 256, "texture_height": 256 },
      "bones": [{ "name": "head" }]
    },
    {
      "description": { "identifier": "geometry.animated_face_persona-x", "texture_width": 32, "texture_height": 64 },
      "bones": [
        { "name": "hat", "parent": "head", "poly_mesh": {
          "normalized_uvs": true,
          "normals": [[0, 0, -1], [0, 0, 1]],
          "positions": [[0, 0, 0]],
          "uvs": [[0.25, 0.5], [0.5, 0.5], [0.5, 0.625], [0.25, 0.625], [0.75, 0.5], [1.0, 0.625]],
          "polys": [
            [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3]],
            [[0, 1, 4], [0, 1, 5], [0, 1, 5], [0, 1, 4]]
          ]
        }},
        { "name": "head", "parent": "body", "poly_mesh": {
          "normalized_uvs": true,
          "normals": [[0, 0, -1], [0, 0, 1]],
          "positions": [[0, 0, 0]],
          "uvs": [[0.25, 0.75], [0.5, 0.75], [0.5, 0.875], [0.25, 0.875], [0.75, 0.75], [1.0, 0.875]],
          "polys": [
            [[0, 1, 4], [0, 1, 5], [0, 1, 5], [0, 1, 4]],
            [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3]]
          ]
        }}
      ]
    }
  ]
}"#;

#[test]
fn the_animated_face_mesh_gives_the_front_of_head_and_hat_from_the_top() {
    let head = GeometryHeadLocator::locate_animated_face(ANIMATED_FACE).unwrap();

    assert_eq!(head, HeadUv::animated_face());
}

#[test]
fn a_geometry_without_an_animated_face_mesh_gives_no_animated_face() {
    assert_eq!(GeometryHeadLocator::locate_animated_face(CLASSIC_112), None);
    assert_eq!(GeometryHeadLocator::locate_animated_face("null"), None);
}

#[test]
fn an_animated_face_mesh_without_a_north_polygon_gives_none() {
    let json = ANIMATED_FACE.replace("[0, 0, -1]", "[0, 1, 0]");

    assert_eq!(GeometryHeadLocator::locate_animated_face(&json), None);
}
