use heads_core::structs::{FaceImage, HeadUv};

/// What `FaceReader` made of one skin: the face when one was cut, and where it came from.
pub struct FaceRead {
    pub face: Option<Box<FaceImage>>,
    /// `persona` for a face cut from an animated face image, else `skin`.
    pub source: &'static str,
    /// `animated_face`, `geometry` or `classic`: where the head rectangle came from.
    pub head_source: &'static str,
    /// `Ok`, `SkinNotReadable` or the error of the extractor.
    pub outcome: String,
    pub head: HeadUv,
    pub geometry_json_len: Option<usize>,
}
