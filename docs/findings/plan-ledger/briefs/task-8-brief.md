### Task 8: Texture bridge, waypoint tagger and the two draw detours

**Files:**
- Create: `heads/amml/src/structs/{game_texture.rs, tag_frame.rs, resolved_texture_fns.rs}`, `heads/amml/src/services/{texture_bridge.rs, waypoint_tagger.rs, render_state.rs}`, `heads/amml/src/detours/{mod.rs, render.rs, draw_icon.rs}`
- Test: `heads/amml/tests/services/{waypoint_tagger.rs, render_state.rs}`

**Interfaces:**
- Consumes: `FaceStore<GameTexture>`, `TagCodec`, `QuadScaler`, `HeadsSettings`, `ClientWaypointLayout`, and the Task 2 entries.
- Produces:
  - `GameTexture`: an owned `TexturePtrLayout` and an owned `ResourceLocation` buffer. `Drop` releases the TexturePtr references (R11). It does not unload; unloading is explicit.
  - `ResolvedTextureFns { resource_location_ctor, resource_location_dtor, image_buffer_ctor, upload, get, unload: usize }`: the resolved code addresses of the Task 2 texture entries.
  - `TextureBridge::new(fns: ResolvedTextureFns)`.
  - `fn upload(&self, group: usize, key: FaceKey, face: &FaceImage) -> Option<GameTexture>`: `ImageBuffer.ctorFromDescription`, a copy of the pixels, `uploadTexture`, then `getTexture`.
  - `fn unload(&self, group: usize, texture: GameTexture)`.
  - `TagFrame`: fixed `[Saved; 256]` (waypoint ptr, saved color bytes, saved flag, face actor), `count`, and `active: bool`.
  - `WaypointTagger::tag(list_head: usize, entity_vtable: usize, frame: &mut TagFrame) -> bool`.
  - `WaypointTagger::restore(frame: &mut TagFrame)`.
  - `RenderState { store, receiver, frame, settings, bridge, deferred: Vec<GameTexture> (capacity 256) }`.
  - `fn before_render(&mut self, client: usize)`: drain up to 64 updates, upload under budget, tag.
  - `fn after_render(&mut self, group: usize)`: restore, unload the deferred textures.
  - `fn icon_for(&mut self, packed: u32) -> IconChoice`.
  - `IconChoice::{Face(*const TexturePtrLayout), Vanilla(u32)}`.

- [ ] **Step 1: Write failing tests** on fake waypoint lists (a circular list of `ListNodeLayout` with fake `ClientWaypointLayout` values).
  - `tags_every_visible_waypoint_and_restores_bytes_exactly`: after restore, a byte compare of each waypoint equals the original. This covers waypoints whose colour flag was empty.
  - `invisible_waypoints_are_not_tagged`
  - `entity_waypoint_with_face_decodes_to_face`
  - `plain_waypoint_decodes_to_vanilla_packed_colour`: the original colour, packed through `TagCodec::pack`.
  - `empty_colour_flag_decodes_to_white`
  - `over_capacity_frame_tags_nothing_and_passes_through`: with 257 waypoints `tag` returns false, nothing changes, and `icon_for(x)` gives `Vanilla(x)` for any x.
  - `remove_during_frame_keeps_texture_until_frame_end`: apply `Remove` between `before_render` and `icon_for`. `icon_for` still returns the face pointer, and the texture is in `deferred` until `after_render`.
  - `disabled_setting_tags_nothing`
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement** `RenderState`, `WaypointTagger` and `TextureBridge`.
  - Store updates drained in `before_render` are applied only before tagging, so `Remove` never frees a texture mid-frame. Released textures go to `deferred`.
- [ ] **Step 4: Implement the detours.**
  - `render` (vslot): `(this, ctx, client, owner, pass: i32) -> ()`. Lock state with `try_lock`; on failure call the original only. Call `before_render(client)`, then the original, then `after_render(group)`.
  - `draw_icon` (function, the R1 signature, 17 arguments, hand-written with `GUARD.original_fn::<F>()` as `lightlevels` does). When the frame is not active, call the original unchanged. On `Face(tex)`: pass `tex` as r8, colour `0xFFFFFFFF`, and the rect through `QuadScaler::scale`. On `Vanilla(c)`: pass `c`.
  - Both detours run inside `DetourGuard::run`.
- [ ] **Step 5: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 6: Checkpoint.**

