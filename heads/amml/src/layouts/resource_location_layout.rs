#![allow(unsafe_code)]

use amml_msvc::MsvcString;

/// Mirror of the 26.60 `ResourceLocation` (0x38 bytes).
#[repr(C)]
pub struct ResourceLocationLayout {
    pub file_system: u32,
    pub path: MsvcString,
    pub path_hash: u64,
    pub full_hash: u64,
}

impl ResourceLocationLayout {
    /// An all-zero buffer, ready for the game constructor to build a location in place.
    pub fn zeroed() -> Self {
        // SAFETY: every field is plain data for which all-zero bytes are valid (an empty small string).
        unsafe { std::mem::zeroed() }
    }
}
