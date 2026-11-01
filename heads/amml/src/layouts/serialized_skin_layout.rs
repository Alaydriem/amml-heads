/// Offsets inside the 26.60 `SerializedSkinImpl`, which starts at +0 of the `shared_ptr` element.
///
/// Proven in code (re-confirm findings R10): `mSkinImage` at 0x80. Computed from the LeviLamina 26.60 header, by
/// adding the member sizes in order, and not proven in the game: every other offset here.
pub struct SerializedSkinLayout;

impl SerializedSkinLayout {
    /// `mResourcePatch`, a `std::string` (header order: mId 0x00, mFullId 0x20, mResourcePatch 0x40).
    pub const RESOURCE_PATCH: usize = 0x40;
    /// `mDefaultGeometryName`, a `std::string`.
    pub const DEFAULT_GEOMETRY_NAME: usize = 0x60;
    /// `mSkinAnimatedImages`, a `std::vector<AnimatedImageData>` (mSkinImage 0x80 and mCapeImage 0xB0, 48 bytes each).
    pub const ANIMATED_IMAGES: usize = 0xE0;
    /// `sizeof(AnimatedImageData)`: mType, mAnimationExpression, a 48 byte SkinImage and mFrames, padded to 8.
    pub const ANIMATED_IMAGE_SIZE: usize = 0x40;
    /// `AnimatedImageData::mType`, a 4 byte enum at the start of an element.
    pub const ANIMATED_IMAGE_TYPE: usize = 0;
    /// `persona::AnimatedTextureType::Face`, the `mType` of the animated face image.
    pub const ANIMATED_TYPE_FACE: u32 = 1;
    /// `AnimatedImageData::mImage`, a 48 byte `mce::Image` laid out as `ImageLayout`.
    pub const ANIMATED_IMAGE_IMAGE: usize = 8;
    /// `mIsPremium`: after mGeometryData 0xF8, mGeometryDataMinEngineVersion 0x108, mGeometryDataMutable 0x128,
    /// mAnimationData 0x138, mCapeId 0x158, mPersonaPieces 0x178, mArmSizeType 0x190, mPieceTintColors 0x198,
    /// mSkinColor 0x1D8 and mIsTrustedSkin 0x1E8.
    pub const IS_PREMIUM: usize = 0x1E9;
    /// `mIsPersona`, the byte after `mIsPremium`.
    pub const IS_PERSONA: usize = 0x1EA;
    /// The bytes the reader may touch from the start of the element, up to and including `mIsPersona`.
    pub const READ_SPAN: usize = 0x1EB;
}
