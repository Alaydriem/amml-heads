/// A zeroed, 8-byte aligned buffer standing in for game memory.
pub struct FakeMemory {
    words: Vec<u64>,
}

impl FakeMemory {
    pub fn new(bytes: usize) -> Self {
        Self { words: vec![0; bytes.div_ceil(8)] }
    }

    pub fn addr(&self) -> usize {
        self.words.as_ptr() as usize
    }

    pub fn put<T: Copy>(&mut self, offset: usize, value: T) {
        assert!(offset + size_of::<T>() <= self.words.len() * 8);
        // SAFETY: the assert keeps the write inside the buffer, and the write is unaligned-safe.
        unsafe { (self.words.as_mut_ptr().cast::<u8>().add(offset).cast::<T>()).write_unaligned(value) };
    }

    pub fn get<T: Copy>(&self, offset: usize) -> T {
        assert!(offset + size_of::<T>() <= self.words.len() * 8);
        // SAFETY: the assert keeps the read inside the buffer, and the read is unaligned-safe.
        unsafe { (self.words.as_ptr().cast::<u8>().add(offset).cast::<T>()).read_unaligned() }
    }

    /// Fills the whole buffer with a repeating non-zero pattern.
    pub fn fill_pattern(&mut self, seed: u8) {
        for i in 0..self.words.len() * 8 {
            self.put(i, seed.wrapping_add((i as u8).wrapping_mul(7)) | 1);
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        (0..self.words.len() * 8).map(|i| self.get::<u8>(i)).collect()
    }
}
