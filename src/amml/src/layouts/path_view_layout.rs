/// Mirror of `Core::PathView`: a character pointer and a length.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PathViewLayout {
    pub data: *const u8,
    pub size: usize,
}

impl PathViewLayout {
    pub fn new(text: &str) -> Self {
        Self { data: text.as_ptr(), size: text.len() }
    }
}
