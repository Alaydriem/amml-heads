/// Names one version of one player face. The generation changes when the skin changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceKey {
    pub actor: i64,
    pub generation: u32,
}

impl FaceKey {
    pub fn new(actor: i64, generation: u32) -> Self {
        Self { actor, generation }
    }
}
