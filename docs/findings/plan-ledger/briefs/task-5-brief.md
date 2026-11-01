### Task 5: Face store, updates and settings (core)

**Files:**
- Create: `heads/core/src/structs/face/{face_key.rs, face_update.rs}`, `heads/core/src/services/face_store.rs`, `heads/core/src/config/{mod.rs, heads_settings.rs}`
- Test: `heads/core/tests/services/face_store.rs`, `heads/core/tests/config/heads_settings.rs`

**Interfaces:**
- Consumes: `FaceImage` (Task 3).
- Produces:
  - `FaceUpdate::{Add { uuid: [u8; 16], actor: i64, face: Box<FaceImage> }, Skin { uuid: [u8; 16], face: Box<FaceImage> }, Remove { uuid: [u8; 16] }, Clear}`.
  - `FaceKey { actor: i64, generation: u32 }`.
  - `FaceStore<T>::new(capacity: usize, uploads_per_frame: usize)`.
  - `fn apply(&mut self, update: FaceUpdate, released: &mut Vec<T>)`.
  - `fn begin_frame(&mut self)`.
  - `fn next_upload(&mut self) -> Option<(FaceKey, &FaceImage)>`.
  - `fn attach(&mut self, key: FaceKey, texture: T) -> Option<T>`. It returns the texture when the key is stale.
  - `fn texture_for(&mut self, actor: i64) -> Option<&T>`. It marks the entry as recently used.
  - `fn len(&self) -> usize`.
  - `HeadsSettings { enabled: bool, scale: f32 }` with serde defaults (true, 1.0) and `fn scale(&self) -> f32`, clamped to 1.0..=2.0.

- [ ] **Step 1: Write failing tests.**
  - `evicts_least_recent_past_capacity`: capacity 2. Add actors 1, 2, 3 with textures attached. Len is 2, actor 1 is gone, and its texture is in `released`.
  - `texture_for_refreshes_recency`: add 1 and 2, look up 1, add 3. Actor 2 is evicted.
  - `skin_update_bumps_generation_and_releases_old_texture`.
  - `stale_attach_is_returned`: attach with the old generation after a `Skin` update returns `Some(texture)`.
  - `remove_by_uuid_releases`.
  - `clear_releases_everything`.
  - `upload_budget_per_frame`: 6 pending and budget 4 give 4 from `next_upload`. After `begin_frame`, the next 2 come.
  - `skin_for_unknown_uuid_is_ignored`.
  - `settings_default_and_clamp`.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.**
  - Use a fixed-capacity slab, a recency index (an order counter, with eviction by the minimum), and `uuid -> actor` and `actor -> slot` maps, both sized to `capacity`.
  - Allocation only in `apply`, which runs at packet rate and never per frame.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

