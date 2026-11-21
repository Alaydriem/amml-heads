use amml_api::{AnimatedKind, ImageView, SkinRef};
use heads_core::services::{FaceExtractor, GeometryHeadLocator};
use heads_core::structs::{HeadUv, SkinView};

use crate::structs::FaceRead;

/// Cuts the face out of a skin the loader lends for one network event.
///
/// A persona skin gets its face from the animated face image, at the head the geometry names (or the fixed
/// rectangles when the geometry has no readable animated face mesh). Any other skin gets it from the main image, at
/// the head of its geometry or the classic head. The result owns its pixels: nothing lent by the event is kept.
pub struct FaceReader;

impl FaceReader {
    pub fn read(skin: &SkinRef<'_>) -> FaceRead {
        let image = skin.image();
        let view = image.as_ref().map(Self::view);
        let animated_image =
            skin.animated_images().find(|(kind, _)| *kind == AnimatedKind::Face).map(|(_, image)| image);
        let animated = animated_image.as_ref().map(Self::view);
        let (head, animated_head, json_len) = if view.is_some() || animated.is_some() {
            let json = skin.geometry_json();
            let name = skin.default_geometry_name();
            // without the name the geometry head locator is off, and the skin keeps its classic head
            let head = json.zip(name).and_then(|(json, name)| GeometryHeadLocator::locate(json, name));
            let animated_head = json.filter(|_| animated.is_some()).and_then(GeometryHeadLocator::locate_animated_face);
            (head, animated_head, json.map(str::len))
        } else {
            (None, None, None)
        };
        // a persona face is drawn from the animated face image; the fixed rectangles are used only on a persona skin
        // whose geometry has no readable animated face mesh
        let animated_head = animated_head
            .or_else(|| (animated.is_some() && skin.is_persona() == Some(true)).then(HeadUv::animated_face));
        let from_animated = animated
            .as_ref()
            .zip(animated_head.as_ref())
            .map(|(view, head)| FaceExtractor::extract_at(view, head))
            .filter(Result::is_ok);
        let (source, head_source, extracted, used_head) = match from_animated {
            Some(face) => ("persona", "animated_face", Some(face), animated_head),
            None => (
                "skin",
                if head.is_some() { "geometry" } else { "classic" },
                view.as_ref().map(|view| match &head {
                    Some(head) => FaceExtractor::extract_at(view, head),
                    None => FaceExtractor::extract(view),
                }),
                head,
            ),
        };
        let outcome = match &extracted {
            Some(Ok(_)) => "Ok".to_string(),
            Some(Err(error)) => format!("{error:?}"),
            None => "SkinNotReadable".to_string(),
        };
        FaceRead {
            face: extracted.and_then(Result::ok).map(Box::new),
            source,
            head_source,
            outcome,
            head: used_head.unwrap_or_else(HeadUv::classic),
            geometry_json_len: json_len,
        }
    }

    // the pixels stay in the game's memory; the view borrows them for the event
    fn view<'f>(image: &ImageView<'f>) -> SkinView<'f> {
        SkinView::new(image.width, image.height, u32::from(image.format), image.pixels)
    }
}
