#![allow(unsafe_code)]

use std::ffi::c_void;

use amml_msvc::SharedPtr;

use super::ResourceLocationLayout;

/// Mirror of the 26.60 `TexturePtr`: a shared texture and a shared resource location.
#[repr(C)]
pub struct TexturePtrLayout {
    pub texture: SharedPtr<c_void>,
    pub location: SharedPtr<ResourceLocationLayout>,
}

impl TexturePtrLayout {
    /// Two empty shared pointers, ready for the game to fill.
    pub fn empty() -> Self {
        // SAFETY: an all-zero shared_ptr is the empty pointer.
        unsafe { std::mem::zeroed() }
    }

    /// Whether the game left the texture pointer empty.
    pub fn is_empty(&self) -> bool {
        self.texture.get().is_none()
    }

    /// Drops the use each shared pointer holds, location first, as the game destructor does.
    ///
    /// # Safety
    /// Each pointer must be empty or hold a live use, and this must run at most once per filled value.
    pub unsafe fn release(&mut self) {
        // SAFETY: the caller guarantees both pointers are empty or live.
        unsafe {
            self.location.release();
            self.texture.release();
        }
    }
}
