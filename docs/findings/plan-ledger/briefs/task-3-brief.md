### Task 3: Workspace scaffold and the face extractor (core)

**Files:**
- Create: `Cargo.toml`, `mise.toml`, `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml` (copy from `../zoom`). Leave out the `install-mod` task and `mise.local.toml`. Copy the zoom `CLAUDE.md` with the names changed.
- Create: `heads/core/Cargo.toml`, `heads/core/src/lib.rs` (`#![forbid(unsafe_code)]`)
- Create: `heads/core/src/structs/face/{mod.rs, face_image.rs, skin_view.rs}`, `heads/core/src/errors/{mod.rs, face_error.rs}`, `heads/core/src/services/{mod.rs, face_extractor.rs}`
- Test: `heads/core/tests/integration.rs`, `heads/core/tests/services/face_extractor.rs`

**Interfaces:**
- Produces:
  - `FaceImage { pixels: [u8; 4096] }`, with `FaceImage::SIDE = 32` and `fn pixel(&self, x: u32, y: u32) -> [u8; 4]`.
  - `SkinView<'a> { width: u32, height: u32, format: u32, bytes: &'a [u8] }`.
  - `FaceError::{UnsupportedFormat(u32), UnsupportedSize { width: u32, height: u32 }, ShortBuffer { need: usize, have: usize }}` (thiserror).
  - `FaceExtractor::extract(skin: &SkinView) -> Result<FaceImage, FaceError>`.

- [ ] **Step 1: Write failing tests** in `tests/services/face_extractor.rs`. Use a helper that builds a skin, filling the face region with one colour and the hat region with another.
  - `face_without_hat_upscales_nearest`: 64x64 skin, face red, hat alpha 0. Every output pixel is red. Pixel (0,0) and pixel (3,3) come from source (8,8). Pixel (4,0) comes from (9,8).
  - `opaque_hat_covers_face`: hat blue with alpha 255. Every pixel is blue.
  - `half_alpha_hat_blends`: face (200,0,0,255), hat (0,0,200,128). The output is (100,0,100,255) ±1 per channel.
  - `accepts_64x32`: a 64x32 skin works the same as 64x64.
  - `scales_128`: 128x128 skin with the face at (16..32,16..32). Output pixel (0,0) comes from source (16,16).
  - `rejects_rgb8`: format 3 gives `UnsupportedFormat(3)`.
  - `rejects_width_96`: gives `UnsupportedSize`.
  - `rejects_short_buffer`: a 64x64 skin with a 100-byte buffer gives `ShortBuffer { need: 16384, have: 100 }`.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL (the types do not exist).
- [ ] **Step 3: Implement `FaceExtractor::extract`.**
  - Accepted: format 4 only; width 64, 128 or 256; height equal to the width or half of it.
  - `s = width / 64`.
  - Sample the face at `(8s + x*s/4, 8s + y*s/4)` and the hat at `(40s + ...)`, for x, y in 0..32.
  - Blend source-over in integer arithmetic. The output alpha is 255 when the face is opaque.
- [ ] **Step 4: Run** `mise run test`. Expected: PASS. Run `mise run check`. Expected: clean.
- [ ] **Step 5: Checkpoint** (ledger `task-3.md`).

