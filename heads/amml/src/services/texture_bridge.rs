#![allow(unsafe_code)]

use amml_msvc::MsvcHeap;
use heads_core::structs::FaceImage;

use crate::layouts::{ImageDescriptionLayout, PathViewLayout, ResourceLocationLayout, TexturePtrLayout};
use crate::structs::{GameTexture, ResolvedTextureFns, TextureLayout};

type LocationCtorFn =
    unsafe extern "C" fn(*mut ResourceLocationLayout, *const PathViewLayout) -> *mut ResourceLocationLayout;
type ImageBufferCtorFn = unsafe extern "C" fn(*mut usize, *const ImageDescriptionLayout) -> *mut usize;
type BlobDeleterFn = unsafe extern "C" fn(*mut u8);
type UploadFn = unsafe extern "C" fn(usize, *const ResourceLocationLayout, *mut usize) -> usize;
type GetFn = unsafe extern "C" fn(usize, *mut TexturePtrLayout, *const ResourceLocationLayout, u8, u64, u32) -> usize;
type UnloadFn = unsafe extern "C" fn(usize, *const ResourceLocationLayout, u8) -> u8;

/// Uploads faces into the game texture group and unloads them again.
///
/// Textures are named from a fixed pool of `POOL` names, `heads/slot_<i>`. The game keeps a map node for
/// every name it has ever seen, so reusing names keeps that map bounded. A name returns to the pool on unload.
pub struct TextureBridge {
    fns: ResolvedTextureFns,
    layout: TextureLayout,
    free: Vec<u8>,
    locations: Vec<Option<Box<ResourceLocationLayout>>>,
    page_location: Option<Box<ResourceLocationLayout>>,
}

impl TextureBridge {
    pub const POOL: usize = 128;
    const PAGE_SIDE: u32 = 256;
    // the chat glyph page the font system loads by this name
    const PAGE_NAME: &'static str = "font/glyph_E1";

    pub fn new(fns: ResolvedTextureFns, layout: TextureLayout) -> Self {
        Self {
            fns,
            layout,
            free: (0..Self::POOL as u8).rev().collect(),
            locations: (0..Self::POOL).map(|_| None).collect(),
            page_location: None,
        }
    }

    /// How many resource locations are built.
    pub fn locations_built(&self) -> usize {
        self.locations.iter().flatten().count()
    }

    /// How many names are free.
    pub fn free_names(&self) -> usize {
        self.free.len()
    }

    /// Uploads the 256x256 RGBA chat glyph page under `font/glyph_E1`. False when the pixels are not that size or
    /// the game gives no buffer.
    pub fn upload_page(&mut self, group: usize, pixels: &[u8]) -> bool {
        if pixels.len() != (Self::PAGE_SIDE * Self::PAGE_SIDE * 4) as usize {
            return false;
        }
        let location = self.page_location();
        self.upload_pixels(group, location, Self::PAGE_SIDE, pixels)
    }

    /// Uploads a face under a free name. `None` when the pool is empty or the game does not give the texture back.
    pub fn upload(&mut self, group: usize, face: &FaceImage) -> Option<GameTexture> {
        let slot = self.free.pop()?;
        let location = self.location(slot);
        if !self.upload_pixels(group, location, FaceImage::SIDE, &face.pixels) {
            self.free.push(slot);
            return None;
        }

        let mut ptr = Box::new(TexturePtrLayout::empty());
        // SAFETY: `get` is the resolved getTexture with this signature; the group plus `group_base` is the base the
        // game expects, `ptr` is a writable empty TexturePtr and `location` outlives the call.
        unsafe {
            let get: GetFn = std::mem::transmute::<usize, GetFn>(self.fns.get);
            get(group + self.layout.group_base(), &mut *ptr, location, 0, 0, 0);
        }
        let texture = GameTexture::new(ptr, slot);
        // SAFETY: `texture` is a view of the pointer the game just filled, or an empty one.
        if unsafe { (*texture.texture_ptr()).is_empty() } {
            self.unload_name(group, location);
            self.free.push(slot);
            return None;
        }
        Some(texture)
    }

    /// Unloads the texture from the group, releases it and gives its name back.
    pub fn unload(&mut self, group: usize, texture: GameTexture) {
        let location = self.location(texture.slot());
        self.unload_name(group, location);
        self.forget(texture);
    }

    /// Releases a texture without unloading it, and gives its name back.
    pub fn forget(&mut self, texture: GameTexture) {
        self.free.push(texture.slot());
    }

    /// Frees the path strings of the resource locations on the game heap.
    pub fn shutdown(&mut self, heap: &MsvcHeap) {
        if let Some(mut location) = self.page_location.take() {
            // SAFETY: the path string was built by the game constructor on the game heap.
            unsafe { location.path.release(heap) };
        }
        for entry in &mut self.locations {
            if let Some(mut location) = entry.take() {
                // SAFETY: the path string was built by the game constructor on the game heap.
                unsafe { location.path.release(heap) };
            }
        }
    }

    fn upload_pixels(&self, group: usize, location: *const ResourceLocationLayout, side: u32, pixels: &[u8]) -> bool {
        let description = ImageDescriptionLayout::face(side);
        let mut buffer = [0usize; TextureLayout::BUFFER_CAPACITY_WORDS];
        // SAFETY: `image_buffer_ctor` is the resolved ImageBuffer(ImageDescription const&) with this signature, and
        // `buffer` holds the `buffer_words` words it constructs into (`TextureLayout::new` keeps them within it).
        unsafe {
            let ctor: ImageBufferCtorFn = std::mem::transmute::<usize, ImageBufferCtorFn>(self.fns.image_buffer_ctor);
            ctor(buffer.as_mut_ptr(), &description);
        }
        let data = buffer[self.layout.data_word()] as *mut u8;
        if data.is_null() {
            return false;
        }
        if buffer[self.layout.size_word()] != pixels.len() {
            let deleter = buffer[self.layout.deleter_word()];
            if deleter != 0 {
                // SAFETY: a non-zero deleter is the blob deleter the constructor stored for `data`.
                unsafe { std::mem::transmute::<usize, BlobDeleterFn>(deleter)(data) };
            }
            return false;
        }
        // SAFETY: the size check above shows `data` holds exactly the image bytes.
        unsafe { std::ptr::copy_nonoverlapping(pixels.as_ptr(), data, pixels.len()) };
        // SAFETY: `upload` is the resolved uploadTexture with this signature. The callee frees the blob, so
        // `buffer` is not destroyed here.
        unsafe {
            let upload: UploadFn = std::mem::transmute::<usize, UploadFn>(self.fns.upload);
            upload(group, location, buffer.as_mut_ptr());
        }
        true
    }

    fn unload_name(&self, group: usize, location: *const ResourceLocationLayout) {
        // SAFETY: `unload` is the resolved unloadTexture with this signature and `location` is a live location.
        unsafe {
            let unload: UnloadFn = std::mem::transmute::<usize, UnloadFn>(self.fns.unload);
            unload(group, location, 0);
        }
    }

    // the resource location of the glyph page, built by the game constructor on first use and kept
    fn page_location(&mut self) -> *const ResourceLocationLayout {
        let ctor_address = self.fns.resource_location_ctor;
        if self.page_location.is_none() {
            let mut location = Box::new(ResourceLocationLayout::zeroed());
            let view = PathViewLayout::new(Self::PAGE_NAME);
            // SAFETY: `ctor_address` is the resolved ResourceLocation(PathView) with this signature; it builds into
            // the zeroed buffer, whose size `TextureLayout::new` matched to the entry, and copies the name.
            unsafe {
                let ctor: LocationCtorFn = std::mem::transmute::<usize, LocationCtorFn>(ctor_address);
                ctor(&mut *location, &view);
            }
            self.page_location = Some(location);
        }
        self.page_location.as_deref().map_or(std::ptr::null(), std::ptr::from_ref)
    }

    // the resource location for a slot, built by the game constructor on first use and kept
    fn location(&mut self, slot: u8) -> *const ResourceLocationLayout {
        let ctor_address = self.fns.resource_location_ctor;
        let entry = &mut self.locations[slot as usize];
        if entry.is_none() {
            let mut location = Box::new(ResourceLocationLayout::zeroed());
            let name = format!("heads/slot_{slot}");
            let view = PathViewLayout::new(&name);
            // SAFETY: `ctor_address` is the resolved ResourceLocation(PathView) with this signature; it builds into
            // the zeroed buffer, whose size `TextureLayout::new` matched to the entry, and copies the name.
            unsafe {
                let ctor: LocationCtorFn = std::mem::transmute::<usize, LocationCtorFn>(ctor_address);
                ctor(&mut *location, &view);
            }
            *entry = Some(location);
        }
        entry.as_deref().map_or(std::ptr::null(), std::ptr::from_ref)
    }
}
