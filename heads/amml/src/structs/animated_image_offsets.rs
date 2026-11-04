use crate::layouts::ImageLayout;

/// Where the animated images of a `SerializedSkinImpl` are, read from the field entries when the mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimatedImageOffsets {
    /// `mSkinAnimatedImages` (a `std::vector<AnimatedImageData>`) inside `SerializedSkinImpl`.
    pub list: usize,
    /// `sizeof(AnimatedImageData)`, the stride of the vector.
    pub stride: usize,
    /// `AnimatedImageData::mType`, a 4 byte enum.
    pub kind: usize,
    /// `AnimatedImageData::mImage`, an `mce::Image` laid out as `ImageLayout`.
    pub image: usize,
}

impl AnimatedImageOffsets {
    /// `persona::AnimatedTextureType::Face`, the `mType` of the animated face image.
    pub const TYPE_FACE: u32 = 1;

    /// Whether the type and the image of one element fit inside the stride, so a read never leaves the element.
    pub fn is_usable(&self) -> bool {
        let kind_end = self.kind.checked_add(std::mem::size_of::<u32>());
        let image_end = self.image.checked_add(std::mem::size_of::<ImageLayout>());
        self.stride != 0
            && kind_end.is_some_and(|end| end <= self.stride)
            && image_end.is_some_and(|end| end <= self.stride)
    }

    pub fn new(list: usize, stride: usize, kind: usize, image: usize) -> Self {
        Self { list, stride, kind, image }
    }
}
