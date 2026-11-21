use amml_api::TextureId;
use heads_core::services::FaceStore;
use heads_core::structs::{FaceImage, FaceKey, FaceUpdate};

/// The faces of the players in the world and the texture each one owns once uploaded.
///
/// Bound: `CAPACITY` faces; adding past it evicts the least recently drawn face and hands its texture back. Textures
/// that are no longer wanted wait in `released` until `take_released`; the caller unloads them.
pub struct FaceBoard {
    store: FaceStore<TextureId>,
    released: Vec<TextureId>,
}

impl FaceBoard {
    pub const CAPACITY: usize = 128;

    pub fn new() -> Self {
        Self {
            // one frame budget covers a whole list of faces
            store: FaceStore::new(Self::CAPACITY, Self::CAPACITY),
            released: Vec::with_capacity(Self::CAPACITY),
        }
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn apply(&mut self, update: FaceUpdate) {
        self.store.apply(update, &mut self.released);
    }

    /// The textures no face wants any more, leaving none behind.
    pub fn take_released(&mut self) -> Vec<TextureId> {
        std::mem::take(&mut self.released)
    }

    pub fn has_pending_upload(&self) -> bool {
        self.store.has_pending_upload()
    }

    /// Copies every face that still needs a texture into `out`.
    pub fn take_pending(&mut self, out: &mut Vec<(FaceKey, Box<FaceImage>)>) {
        self.store.begin_frame();
        while let Some((key, face)) = self.store.next_upload() {
            out.push((key, Box::new(face.clone())));
        }
    }

    /// Drops every texture id (the loader freed them) and queues every face for upload. Gives the faces that held one.
    pub fn invalidate_textures(&mut self) -> usize {
        self.store.invalidate_textures()
    }

    pub fn retry_upload(&mut self, key: FaceKey) {
        self.store.retry_upload(key);
    }

    /// Stores the texture made for `key`; a texture nobody wants any more comes back.
    pub fn attach(&mut self, key: FaceKey, texture: TextureId) -> Option<TextureId> {
        self.store.attach(key, texture)
    }

    /// The texture of a player, marking the face as recently drawn.
    pub fn texture_for(&mut self, actor: i64) -> Option<TextureId> {
        self.store.texture_for(actor).copied()
    }
}

impl Default for FaceBoard {
    fn default() -> Self {
        Self::new()
    }
}
