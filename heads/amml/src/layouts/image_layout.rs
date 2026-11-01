/// Mirror of the 26.60 `mce::Image` (0x30 bytes): the format, the size and a `Blob` of pixels.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ImageLayout {
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub usage: u8,
    pub alpha_usage: u8,
    pub reserved: [u8; 6],
    pub blob_deleter: usize,
    pub blob_data: *const u8,
    pub blob_size: usize,
}
