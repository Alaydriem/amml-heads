use super::FaceImage;

/// A change to the set of known player faces, sent from the packet side to the render side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaceUpdate {
    Add { uuid: [u8; 16], actor: i64, face: Box<FaceImage> },
    Skin { uuid: [u8; 16], face: Box<FaceImage> },
    Remove { uuid: [u8; 16] },
    Clear,
}
