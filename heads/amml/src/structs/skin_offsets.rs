use super::AnimatedImageOffsets;

/// The members of `SerializedSkinImpl` the mod reads, read from the field entries when the mod enables. Each is
/// an offset from the element that the `shared_ptr` of a `SerializedSkinRef` points to. The members of a side
/// feature are `None` when that feature is off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkinOffsets {
    /// `mSkinImage` (an `mce::Image`).
    pub image: usize,
    /// `mGeometryData` (a `Json::Value`).
    pub geometry_data: usize,
    /// `mResourcePatch` (a `std::string`); skin diagnostics only.
    pub resource_patch: Option<usize>,
    /// `mDefaultGeometryName` (a `std::string`); the geometry head locator and the diagnostics.
    pub default_geometry_name: Option<usize>,
    /// `mIsPremium` (a bool); skin diagnostics only.
    pub is_premium: Option<usize>,
    /// `mIsPersona` (a bool); the persona animated face and the diagnostics.
    pub is_persona: Option<usize>,
    /// `mSkinAnimatedImages` and the layout of one of its elements; the persona animated face and the diagnostics.
    pub animated: Option<AnimatedImageOffsets>,
}

impl SkinOffsets {
    pub fn new(
        image: usize,
        geometry_data: usize,
        resource_patch: Option<usize>,
        default_geometry_name: Option<usize>,
        is_premium: Option<usize>,
        is_persona: Option<usize>,
        animated: Option<AnimatedImageOffsets>,
    ) -> Self {
        Self { image, geometry_data, resource_patch, default_geometry_name, is_premium, is_persona, animated }
    }

    /// The offsets with every side feature off.
    pub fn new_required(image: usize, geometry_data: usize) -> Self {
        Self::new(image, geometry_data, None, None, None, None, None)
    }
}
