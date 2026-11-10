/// The resolved code addresses of the texture group functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedTextureFns {
    /// `ResourceLocation(Core::PathView)`.
    pub resource_location_ctor: usize,
    /// `ImageBuffer(ImageDescription const&)`.
    pub image_buffer_ctor: usize,
    /// `TextureGroup::uploadTexture(ResourceLocation const&, cg::ImageBuffer)`.
    pub upload: usize,
    /// `TextureGroupBase::getTexture`, called on the group plus `TextureLayout::group_base`.
    pub get: usize,
    /// `TextureGroup::unloadTexture(ResourceLocation const&, bool)`.
    pub unload: usize,
}
