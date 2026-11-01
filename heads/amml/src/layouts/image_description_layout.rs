/// Mirror of the 26.60 `cg::ImageDescription`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageDescriptionLayout {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub color_space: u8,
    pub image_type: u8,
    pub alpha_usage: u8,
    pub reserved: u8,
    pub depth: u32,
}

impl ImageDescriptionLayout {
    pub const RGBA8: u32 = 0x1C;
    const SRGB: u8 = 1;

    /// A square RGBA8 sRGB description, as `addEmptyTexture` builds by default.
    pub fn face(side: u32) -> Self {
        Self {
            width: side,
            height: side,
            format: Self::RGBA8,
            color_space: Self::SRGB,
            image_type: 0,
            alpha_usage: 0,
            reserved: 0,
            depth: 1,
        }
    }
}
