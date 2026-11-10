use heads_core::structs::HeadUv;

/// What the geometry of one skin gave: the raw JSON, when it could be read, the head found in it and, when asked
/// for, the head of its animated face mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct GeometryRead {
    pub head: Option<HeadUv>,
    pub animated_head: Option<HeadUv>,
    pub json: Option<String>,
}
