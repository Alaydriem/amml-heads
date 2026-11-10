use alaydriem_heads::layouts::ImageLayout;

use super::FakeMemory;

pub const SKIN_IMAGE_OFFSET: usize = 0x80;

/// A fake `SerializedSkinImpl` with its image and pixel blob, reachable through a fake `shared_ptr`.
pub struct FakeSkin {
    implementation: FakeMemory,
    shared: FakeMemory,
    blob: Vec<u8>,
    animated: FakeMemory,
    animated_blob: Vec<u8>,
}

impl FakeSkin {
    /// A skin whose pixels are all `pixel`, with the blob size set to `blob_len`.
    pub fn new(width: u32, height: u32, format: u32, blob_len: usize, pixel: [u8; 4]) -> Self {
        let blob: Vec<u8> = (0..blob_len).map(|i| pixel[i % 4]).collect();
        let mut implementation = FakeMemory::new(0x200);
        implementation.put(
            SKIN_IMAGE_OFFSET,
            ImageLayout {
                format,
                width,
                height,
                depth: 1,
                usage: 0,
                alpha_usage: 0,
                reserved: [0; 6],
                blob_deleter: 0,
                blob_data: blob.as_ptr(),
                blob_size: blob_len,
            },
        );
        let mut shared = FakeMemory::new(16);
        shared.put(0, implementation.addr());
        Self { implementation, shared, blob, animated: FakeMemory::new(8), animated_blob: Vec::new() }
    }

    pub fn rgba(width: u32, height: u32, pixel: [u8; 4]) -> Self {
        Self::new(width, height, 4, (width * height * 4) as usize, pixel)
    }

    /// The address of the fake `shared_ptr`, as a skin reference.
    pub fn skin_ref(&self) -> usize {
        self.shared.addr()
    }

    /// The raw element pointer the fake `shared_ptr` holds.
    pub fn element(&self) -> usize {
        self.implementation.addr()
    }

    /// Sets the premium and persona flags at their 26.60 offsets.
    pub fn mark(&mut self, premium: bool, persona: bool) {
        self.implementation.put(0x1E9, u8::from(premium));
        self.implementation.put(0x1EA, u8::from(persona));
    }

    /// Sets mDefaultGeometryName (a short inline string) and the length of mResourcePatch.
    pub fn set_strings(&mut self, geometry: &str, patch_len: usize) {
        let mut bytes = [0u8; 16];
        bytes[..geometry.len()].copy_from_slice(geometry.as_bytes());
        self.implementation.put(0x60, bytes);
        self.implementation.put(0x60 + 0x10, geometry.len());
        self.implementation.put(0x60 + 0x18, 15usize);
        self.implementation.put(0x40 + 0x10, patch_len);
        self.implementation.put(0x40 + 0x18, patch_len.max(15));
    }

    /// Sets mSkinAnimatedImages to elements of the given types.
    pub fn set_animated(&mut self, types: &[u32]) {
        self.animated = FakeMemory::new(0x40 * types.len().max(1));
        for (index, kind) in types.iter().enumerate() {
            self.animated.put(index * 0x40, *kind);
        }
        let first = self.animated.addr();
        self.implementation.put(0xE0, first);
        self.implementation.put(0xE8, first + 0x40 * types.len());
    }

    /// Sets mSkinAnimatedImages to one RGBA face image (type 1) of `width` x `height` pixels, all transparent but
    /// for the painted squares, each `(x, y, side, pixel)`.
    pub fn set_animated_face(&mut self, width: u32, height: u32, squares: &[(usize, usize, usize, [u8; 4])]) {
        self.animated_blob = vec![0; (width * height * 4) as usize];
        for (x, y, side, pixel) in squares {
            for row in *y..y + side {
                for column in *x..x + side {
                    let at = (row * width as usize + column) * 4;
                    self.animated_blob[at..at + 4].copy_from_slice(pixel);
                }
            }
        }
        self.animated = FakeMemory::new(0x40);
        self.animated.put(0, 1u32);
        self.animated.put(
            8,
            ImageLayout {
                format: 4,
                width,
                height,
                depth: 1,
                usage: 0,
                alpha_usage: 0,
                reserved: [0; 6],
                blob_deleter: 0,
                blob_data: self.animated_blob.as_ptr(),
                blob_size: self.animated_blob.len(),
            },
        );
        let first = self.animated.addr();
        self.implementation.put(0xE0, first);
        self.implementation.put(0xE8, first + 0x40);
    }

    /// Paints a square of `side` pixels at (x, y) with one colour; the image is `width` pixels wide.
    pub fn paint(&mut self, x: usize, y: usize, side: usize, pixel: [u8; 4]) {
        let width = self.implementation.get::<ImageLayout>(SKIN_IMAGE_OFFSET).width as usize;
        for row in y..y + side {
            for column in x..x + side {
                let at = (row * width + column) * 4;
                self.blob[at..at + 4].copy_from_slice(&pixel);
            }
        }
    }

    pub fn blob(&self) -> &[u8] {
        &self.blob
    }
}
