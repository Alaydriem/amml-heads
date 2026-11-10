use serde_json::Value;

use crate::structs::{HeadUv, UvRect};

/// Finds the head and hat of a skin geometry, in the 1.12 or the 1.8 geometry format.
pub struct GeometryHeadLocator;

impl GeometryHeadLocator {
    // largest geometry text read
    const MAX_JSON: usize = 1024 * 1024;
    const DEFAULT_TEXTURE: u32 = 64;
    const MAX_TEXTURE: f64 = 16384.0;
    const ANIMATED_FACE_PREFIX: &'static str = "geometry.animated_face";
    // a normal faces north when its z is at most this
    const NORTH_Z: f64 = -0.9;

    /// The head of the geometry named `geometry_name`, or of the first geometry when none has that name.
    /// `None` when the text is too large or not a geometry, or the geometry has no usable head cube.
    pub fn locate(json: &str, geometry_name: &str) -> Option<HeadUv> {
        if json.len() > Self::MAX_JSON {
            return None;
        }
        let root: Value = serde_json::from_str(json).ok()?;
        let root = root.as_object()?;
        if let Some(entries) = root.get("minecraft:geometry").and_then(Value::as_array) {
            let entry = entries
                .iter()
                .find(|entry| Self::identifier(entry) == Some(geometry_name))
                .or_else(|| entries.first())?;
            let description = entry.get("description");
            let width = Self::texture(description.and_then(|d| d.get("texture_width")))?;
            let height = Self::texture(description.and_then(|d| d.get("texture_height")))?;
            return Self::head(entry.get("bones")?, width, height);
        }
        let named = root.iter().find(|(key, _)| key.split(':').next() == Some(geometry_name));
        let (_, geometry) = named.or_else(|| root.iter().find(|(key, _)| key.starts_with("geometry.")))?;
        let width = Self::texture(geometry.get("texturewidth"))?;
        let height = Self::texture(geometry.get("textureheight"))?;
        Self::head(geometry.get("bones")?, width, height)
    }

    /// The head of the `geometry.animated_face*` mesh of a persona geometry, from the front (north) polygons of its
    /// `head` and `hat` poly meshes. `None` when the text is too large or not a geometry, or the mesh has no front
    /// polygon on its head.
    pub fn locate_animated_face(json: &str) -> Option<HeadUv> {
        if json.len() > Self::MAX_JSON {
            return None;
        }
        let root: Value = serde_json::from_str(json).ok()?;
        let entry = root
            .get("minecraft:geometry")?
            .as_array()?
            .iter()
            .find(|entry| Self::identifier(entry).is_some_and(|id| id.starts_with(Self::ANIMATED_FACE_PREFIX)))?;
        let description = entry.get("description");
        let width = Self::texture(description.and_then(|d| d.get("texture_width")))?;
        let height = Self::texture(description.and_then(|d| d.get("texture_height")))?;
        let bones = entry.get("bones")?.as_array()?;
        let face = Self::mesh_front(Self::bone(bones, "head")?, width, height)?;
        let hat = Self::bone(bones, "hat").and_then(|bone| Self::mesh_front(bone, width, height));
        Some(HeadUv::new(face, hat, width, height))
    }

    // the bounds, in texture pixels from the top left, of the UVs of every polygon whose normals all face north;
    // poly mesh UVs start at the bottom left, and are fractions of the texture when `normalized_uvs` is set
    fn mesh_front(bone: &Value, texture_width: u32, texture_height: u32) -> Option<UvRect> {
        let mesh = bone.get("poly_mesh")?;
        let normalized = mesh.get("normalized_uvs").and_then(Value::as_bool).unwrap_or(false);
        let (scale_u, scale_v) = if normalized { (texture_width as f64, texture_height as f64) } else { (1.0, 1.0) };
        let normals = mesh.get("normals")?.as_array()?;
        let uvs = mesh.get("uvs")?.as_array()?;
        let mut bounds = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
        for poly in mesh.get("polys")?.as_array()? {
            let Some(vertices) = poly.as_array() else { continue };
            let mut corners = Vec::with_capacity(vertices.len());
            for vertex in vertices {
                let indices = vertex.as_array()?;
                let normal = normals.get(Self::index(indices.get(1)?)?)?.as_array()?;
                if normal.get(2)?.as_f64()? > Self::NORTH_Z {
                    corners.clear();
                    break;
                }
                let uv = uvs.get(Self::index(indices.get(2)?)?)?.as_array()?;
                corners.push((uv.first()?.as_f64()? * scale_u, (*uv.get(1)?).as_f64()? * scale_v));
            }
            for (u, v) in corners {
                let y = texture_height as f64 - v;
                bounds = [bounds[0].min(u), bounds[1].min(y), bounds[2].max(u), bounds[3].max(y)];
            }
        }
        let rect = UvRect::new(
            bounds[0] as f32,
            bounds[1] as f32,
            (bounds[2] - bounds[0]) as f32,
            (bounds[3] - bounds[1]) as f32,
        );
        let usable = [rect.x, rect.y, rect.w, rect.h].iter().all(|value| value.is_finite())
            && rect.x >= 0.0
            && rect.y >= 0.0
            && rect.w > 0.0
            && rect.h > 0.0;
        usable.then_some(rect)
    }

    fn index(value: &Value) -> Option<usize> {
        usize::try_from(value.as_u64()?).ok()
    }

    fn identifier(entry: &Value) -> Option<&str> {
        entry.get("description")?.get("identifier")?.as_str()
    }

    // a texture side; 64 when absent, `None` when present but not a positive finite number
    fn texture(value: Option<&Value>) -> Option<u32> {
        let Some(value) = value else { return Some(Self::DEFAULT_TEXTURE) };
        let side = value.as_f64()?;
        (side.is_finite() && (1.0..=Self::MAX_TEXTURE).contains(&side)).then_some(side as u32)
    }

    fn head(bones: &Value, texture_width: u32, texture_height: u32) -> Option<HeadUv> {
        let bones = bones.as_array()?;
        let face = Self::front(Self::bone(bones, "head")?)?;
        let hat = Self::bone(bones, "hat").and_then(Self::front);
        Some(HeadUv::new(face, hat, texture_width, texture_height))
    }

    fn bone<'a>(bones: &'a [Value], name: &str) -> Option<&'a Value> {
        bones.iter().find(|bone| {
            bone.get("name").and_then(Value::as_str).is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
    }

    // the front (north) face of the first cube of the bone that has a size
    fn front(bone: &Value) -> Option<UvRect> {
        let cube = bone.get("cubes")?.as_array()?.iter().find(|cube| cube.get("size").is_some())?;
        let size = Self::numbers(cube.get("size")?, 3)?;
        let uv = cube.get("uv")?;
        let rect = if uv.is_array() {
            let origin = Self::numbers(uv, 2)?;
            UvRect::new(origin[0] + size[2], origin[1] + size[2], size[0], size[1])
        } else {
            let north = uv.get("north")?;
            let origin = Self::numbers(north.get("uv")?, 2)?;
            let extent = Self::numbers(north.get("uv_size")?, 2)?;
            UvRect::new(origin[0], origin[1], extent[0], extent[1])
        };
        (rect.w > 0.0 && rect.h > 0.0).then_some(rect)
    }

    // the first `count` numbers of an array, every one finite and not negative
    fn numbers(value: &Value, count: usize) -> Option<Vec<f32>> {
        let values = value.as_array()?;
        if values.len() < count {
            return None;
        }
        values
            .iter()
            .take(count)
            .map(|value| {
                value.as_f64().filter(|number| number.is_finite() && *number >= 0.0).map(|number| number as f32)
            })
            .collect()
    }
}
