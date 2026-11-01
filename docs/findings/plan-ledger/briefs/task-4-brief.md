### Task 4: Tag codec and quad scaler (core)

**Files:**
- Create: `heads/core/src/structs/tag/{mod.rs, tag_color.rs}`, `heads/core/src/structs/icon_rect.rs`, `heads/core/src/services/{tag_codec.rs, quad_scaler.rs}`
- Test: `heads/core/tests/services/{tag_codec.rs, quad_scaler.rs}`

**Interfaces:**
- Produces:
  - `TagColor { r: f32, g: f32, b: f32, a: f32 }`.
  - `TagCodec::encode(index: u16) -> TagColor`: R = 0xA5/255, G = (index >> 8)/255, B = (index & 0xFF)/255, A = 1.0.
  - `TagCodec::decode(packed: u32) -> Option<u16>`: `None` unless A = 0xFF and R = 0xA5.
  - `TagCodec::pack(color: TagColor) -> u32`: the vanilla `0xAARRGGBB` packing, with the clamp and the rounding from Task 1 R2.
  - `IconRect { x: f32, y: f32, w: f32, h: f32 }`.
  - `QuadScaler::scale(rect: IconRect, factor: f32) -> IconRect`: keeps the centre; `factor` is clamped to 1.0..=2.0.

- [ ] **Step 1: Write failing tests.**
  - `round_trips_every_index`: for i in 0..=255 and 65535, `decode(pack(encode(i))) == Some(i)`.
  - `vanilla_white_is_not_a_tag`: `pack(1,1,1,1)` decodes to `None`.
  - `pack_matches_vanilla_rounding`: the R2 sample values give the bytes the disassembly computes. Copy 3 sample inputs and outputs into the ledger.
  - `pack_clamps_out_of_range`: (-0.5, 2.0, 0.5, 1.0).
  - `scale_keeps_centre`: (10,20,7,7) at x2 gives (6.5,16.5,14,14).
  - `scale_clamps_factor`: 3.0 is treated as 2.0, and 0.5 as 1.0.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement** the signatures above.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

