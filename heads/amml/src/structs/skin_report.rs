/// What one skin looks like in game memory, for the diagnostic log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkinReport {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub blob_size: usize,
    pub size_matches: bool,
    pub is_persona: bool,
    pub is_premium: bool,
    pub default_geometry: String,
    pub resource_patch_len: usize,
    pub animated_types: Vec<u32>,
}
