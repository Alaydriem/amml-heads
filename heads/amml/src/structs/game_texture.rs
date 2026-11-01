#![allow(unsafe_code)]

use crate::layouts::TexturePtrLayout;

/// A face texture held in the game texture group, under one name from the bridge pool.
///
/// Dropping it releases the references it holds. It does not unload the texture: unloading is explicit, through
/// `TextureBridge::unload`.
pub struct GameTexture {
    ptr: Box<TexturePtrLayout>,
    slot: u8,
}

impl GameTexture {
    pub(crate) fn new(ptr: Box<TexturePtrLayout>, slot: u8) -> Self {
        Self { ptr, slot }
    }

    /// The `TexturePtr` the icon blit reads. It stays valid for as long as this value lives.
    pub fn texture_ptr(&self) -> *const TexturePtrLayout {
        &*self.ptr
    }

    /// The pool name index.
    pub fn slot(&self) -> u8 {
        self.slot
    }
}

impl Drop for GameTexture {
    fn drop(&mut self) {
        // SAFETY: the pointers were filled by the game `getTexture` call, or are still empty, and are released once.
        unsafe { self.ptr.release() };
    }
}
