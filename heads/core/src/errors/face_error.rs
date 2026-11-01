#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FaceError {
    #[error("skin pixel format {0} is not RGBA8")]
    UnsupportedFormat(u32),
    #[error("skin size {width}x{height} is not supported")]
    UnsupportedSize { width: u32, height: u32 },
    #[error("skin buffer holds {have} bytes but {need} are needed")]
    ShortBuffer { need: usize, have: usize },
    #[error("every pixel of the face is transparent")]
    EmptyFace,
}
