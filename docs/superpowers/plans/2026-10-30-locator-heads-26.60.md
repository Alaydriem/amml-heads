# Locator Heads (26.60) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** An AMML client mod, hosted by the Alaydriem Mod Manager, that draws each player's skin face in place of the dot on the vanilla locator bar in Bedrock 26.60.05.

**Architecture:**
- Packet detours (`PlayerListPacket`, `PlayerSkinPacket`) copy each skin's face and hat into a 32x32 RGBA image. They send it through a bounded channel to the render side.
- A detour on `HudLocatorBarRenderer::render` uploads faces as game textures. It also tags each waypoint's colour with an index.
- A detour on the single-caller icon blit (`0x598a9d0`) decodes the tag and swaps the dot texture for the face texture. Vanilla still computes everything else.

**Tech Stack:**
- Rust 2024 (the toolchain zoom pins)
- `amml-sdk`, `amml-msvc`, `amml-game` (path `../amml`)
- `amm-client`, `amm-core` (path `../modmanager-amml`, branch `amml-port`)
- `crossbeam-channel`, Curia logging, Criterion, cargo-nextest, mise

**Spec:** `docs/superpowers/specs/2026-10-30-locator-heads-design.md`. The evidence is in `docs/findings/{locator-bar-render,skin-pipeline,amml-integration}/findings.md`. Read all four before starting.

## Global Constraints

- Target build: 1.26.60.05, BuildId `6ad79ee4-17d51891a1119914`. Test instance: a private copy, `%LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads`, made from `1.26.60.05-standalone` (the current shim and mod manager). Test only in that copy. Never install into, start, or stop a game in any shared instance: other agents use them. Each game process you start: write its PID to `<instance>\locator-heads.pid` (found by path and creation time with `amml/scripts/find-game-pid.ps1`) and stop only that PID. Never write to `%APPDATA%\levilauncher.exe\versions\`. End only the PID you started (`amml/scripts/find-game-pid.ps1`). Never use `taskkill /IM`.
- **Review policy (operator rule):** no per-task reviewer. Each task's own tests are the gate. One whole-branch review after Task 10.
- **No git commits** (operator rule). Each "Checkpoint" step means: run the listed tests, record the result in the task ledger `docs/findings/plan-ledger/task-N.md`, and end the file with "Task Completed".
- Rules from `amml/CLAUDE.md` and `modmanager-amml/CLAUDE.md` apply to every file:
  - One type per file.
  - No free functions.
  - `new()` constructors.
  - No `unwrap`/`expect` outside tests.
  - `// SAFETY:` on every `unsafe` block.
  - Comments on the preceding line, with no plan references.
  - Tests mirror `src/` under one `integration` target, with no zero-value tests.
  - Curia logging with fields and no format arguments.
  - Bounded collections, each with a test that proves the bound.
- Build and test only through mise tasks. Never pipe build, test or derive output into `grep`, `head` or `tail`.
- Recipes follow `amml/docs/recipes.md`:
  - Two independent recipes and a strong check.
  - Every immediate, displacement and branch byte is wildcarded.
  - Patterns end at a call.
  - `notes` on each entry.
  - `mise run lint` gives 0 open findings.
  - `derive` is unchanged for every existing entry on both builds.
- Mod id `alaydriem-locator-heads`. DLL `alaydriem_locator_heads.dll`. Workspace crates `locator-heads-core` (`heads/core`), `locator-heads-amml` (`heads/amml`, lib name `alaydriem_locator_heads`), `locator-heads-bench` (`heads/bench`).
- Fixed values:

  | Value | Setting |
  | --- | --- |
  | Face image | 32x32 RGBA8 |
  | Face store capacity | 128 |
  | Channel capacity | 256 |
  | Uploads per frame | 4 |
  | Tagged waypoints per frame | 256 |
  | `heads.scale` | 1.0 to 2.0, step 0.25, default 1.0 |
  | `heads.enabled` | default true |

## Review Focus

1. **A player leaves, or their skin changes, between the tag and the blit in one frame.** The blit must draw the vanilla dot or the old face, never a freed texture. Task 8 covers this with the test `remove_during_frame_keeps_texture_until_frame_end`.
2. **A skin that is not RGBA8, has an odd size, or has a short blob** (some servers send 64x32 classic skins or empty persona images). The skin is skipped and the vanilla dot stays. Task 3 covers this with the tests `rejects_rgb8`, `rejects_width_96`, `rejects_short_buffer` and `accepts_64x32`.
3. **The world changes or the player disconnects while faces are loaded.** The store and textures are cleared, and no stale actor id maps a face onto a new world's player. Task 9 covers this with the test `exit_level_clears_store_and_unloads_textures`.
4. **A server colours its waypoints with a colour that looks like a tag.** Because every visible waypoint is tagged in the frame, decode never sees an untagged colour. When there are more than 256 waypoints, nothing is tagged and the blit must pass the colour through. Task 8 covers this with the test `over_capacity_frame_tags_nothing_and_passes_through`.
5. **The game is not 26.60, or an entry is missing.** The mod must not hook anything and must log why. Task 9 covers this with the tests `refuses_when_mesh_size_is_not_2660` and `refuses_when_an_entry_is_missing`.

---

### Task 1: Confirm the open reverse-engineering facts (docs only, no product code)

The recipes (Task 2) and the mirrors (Task 7) need numbers that the findings left open. This task measures them on the 26.60.05 exe with `mise run inspect` and `mise run decompile` (in `amml/`). Put the dumps in `docs/findings/re-confirm/`.

**Files:**
- Create: `docs/findings/re-confirm/findings.md`

**Interfaces:**
- Produces: the table below, with every cell filled. Each row gives the RVA or offset, the evidence (dump file and line), and a 26.51 note ("absent", "different", or the value).

| Row | Fact to measure |
| --- | --- |
| R1 | Return type of `0x598a9d0`: does the caller use `rax`/`xmm0` after `0x5746e85`? Do its argument registers and stack slots match `locator-bar-render/findings.md` 2.5? |
| R2 | How the render loop packs colour: `cvttss2si` (truncate) or `cvtss2si` (round), and the clamp order. Give the exact instruction sequence before `0x5746e85`. |
| R3 | `0x3946b0`: which `ResourceLocation` constructor it is (string or `PathView`; is the file system an argument or fixed?). Also the `ResourceLocation` size and its destructor RVA. |
| R4 | `TextureGroup::uploadTexture(ResourceLocation const&, cg::ImageBuffer)`: the RVA. How the by-value `ImageBuffer` is passed (pointer to a caller temp; who destroys it). The `this` adjustment (TextureGroup vs the +0x18 subobject). |
| R5 | `TextureGroup::unloadTexture`: the RVA and `this`. |
| R6 | `cg::ImageBuffer(cg::ImageDescription const&)` constructor RVA. The field order of `cg::ImageDescription` (20 bytes). How to reach the `ImageBuffer` pixel pointer (`mce::Blob` at +0: data pointer offset, size at +0x10). |
| R7 | The `ClientInstance` vslot that returns `shared_ptr<TextureGroup>` (103 by the findings). Check this in the render disasm. |
| R8 | `PlayerListPacket` payload: offset of `mEntries` in the packet, the variant element size, the index byte offset, which index is `AddEntry`, and the `AddEntry` offsets of `mUUID`, `mId` and `mSkin`. Use `inspect fields` on the packet dtor and the 26.60 handler at slot 179. |
| R9 | `PlayerSkinPacket` offsets of `mUUID` and `mSkin` (slot 107 handler). |
| R10 | The `SerializedSkinImpl` offset inside `ThreadOwner<SerializedSkinImpl>`. The offset of `mSkinImage` inside `SerializedSkinImpl`. The `mce::Image` field offsets and the `Blob` data pointer. Read a function that gets an `mce::Image const*` from a `SerializedSkinRef` (a caller of `ClientSkinInfoData::updateSkin`). |
| R11 | `TexturePtr` copy and release: is `amml-msvc` `SharedPtr` enough to add and drop references on the two shared_ptrs, or is a `TexturePtr.dtor` entry needed? |
| R12 | Is the `LocatorBarPacket` handler (slot 38) on the same thread as render? Static evidence only (`ClientInstance` update vs render call chain). Write "unproven" when there is none. Task 10 checks it in game. |

- [ ] **Step 1: Measure R1 to R12** with `mise run inspect -- disasm|fields|xrefs|callers --exe <26.60 exe>` and `mise run decompile`. Each cell needs code evidence. Write "inference" when a cell is inferred.
- [ ] **Step 2: Repeat R3 to R6 and R8 to R10 on the 26.51.01 exe**, only to record whether the layout differs. Do not fix 26.51.
- [ ] **Step 3: Checkpoint.** All cells are filled. Send the file to the operator with SendUserFile. **Stop and report** if R1, R4, R8 or R10 cannot be proven: the draw route or the skin route then needs a design change.

### Task 2: Recipes for the new entries (in `amml/`)

**Files:**
- Create: `amml/recipes/hud_locator_bar_renderer.hcl`, `amml/recipes/texture_group.hcl`, `amml/recipes/resource_location.hcl`, `amml/recipes/image_buffer.hcl`, `amml/recipes/serialized_skin.hcl`, `amml/recipes/player_list_packet.hcl`, `amml/recipes/player_skin_packet.hcl`
- Modify: `amml/recipes/client_network_handler.hcl` (two vslot entries), `amml/recipes/local_player.hcl` (one field entry), `amml/recipes/client_instance.hcl` (one vslot entry)

**Interfaces:**
- Consumes: the Task 1 table.
- Produces: these entry ids, used by Tasks 7 to 9.

| Entry id | Kind | 26.60 value | Notes |
| --- | --- | --- | --- |
| `HudLocatorBarRenderer.vtable` | global | `0xF260170` | Recipes: the `lea` in ctor `0x57447a0` and in dtor `0x576cce0` |
| `HudLocatorBarRenderer.render` | vslot | 16 | Check `in_vtable_of`, range 14..18 |
| `HudLocatorBarRenderer.drawIcon` | function, `hooked = true` | `0x598a9d0` | Recipes: body pattern, and `ref` render then `call` at the blit site |
| `LocalPlayer.mClientLocatorBar` | field | `0xEF0` | Recipes: the LocatorBarPacket handler `add`/`lea`, and the render `+0xF00` read minus `0x10` |
| `ClientInstance.getTextureGroup` | vslot | R7 | |
| `ClientNetworkHandler.handle.PlayerListPacket` | vslot | 179 | Same shape as `...ContainerOpenPacket` |
| `ClientNetworkHandler.handle.PlayerSkinPacket` | vslot | 107 | |
| `TextureGroup.getTexture` | function | `0x541f580` | |
| `TextureGroup.uploadTexture` | function | R4 | |
| `TextureGroup.unloadTexture` | function | R5 | |
| `ResourceLocation.ctor` | function | R3 | |
| `ResourceLocation.dtor` | function | R3 | |
| `ImageBuffer.ctorFromDescription` | function | R6 | |
| `PlayerListPacket.entries` | field | R8 | Also `PlayerListAddEntry.size`, `.mUUID`, `.mId`, `.mSkin`, `PlayerListPacket.variantIndex` |
| `PlayerSkinPacket.mUUID`, `PlayerSkinPacket.mSkin` | field | R9 | |
| `SerializedSkinRef.impl`, `SerializedSkinImpl.mSkinImage` | field | R10 | |

  Use one field entry per offset that moved between 26.51 and 26.60 ("one name, one layout"). Offsets that the headers show as stable may be constants in the mirrors, with an `offset_of!` assert.

- [ ] **Step 1: For each entry, run `mise run inspect -- draft <rva>`**, wildcard the pattern, and check `inspect scan` gives one match. Write the entry with two independent recipes, a check and `notes`.
- [ ] **Step 2: Run** `mise run derive -- --exe <26.60 exe> --recipes recipes --out <tmp> --report <tmp> --compare data/6ad79ee4-17d51891a1119914.hcl`. Expected: every new entry resolves to its table value, and no existing value moves. Run it twice; the two outputs must be identical.
- [ ] **Step 3: Run the same derive on the 26.51.01 exe.** Expected: no existing value moves. New entries either refuse or resolve; record each in the ledger. A wrong 26.51 value is allowed only because the build gate (Task 9) refuses 26.51.
- [ ] **Step 4: Run** `mise run lint -- --exe <exe> --recipes recipes --report <file>` on both builds. Expected: 0 open findings. Run `mise run signature -- --exe <26.60 exe>`. Expected: `HudLocatorBarRenderer.drawIcon` is listed with the R1 registers.
- [ ] **Step 5: Checkpoint.** `mise run check` and `mise run test` in `amml/` pass.

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

### Task 6: Benches

**Files:**
- Create: `heads/bench/Cargo.toml`, `heads/bench/src/lib.rs`, `heads/bench/benches/heads.rs`

- [ ] **Step 1: Write the Criterion benches:**
  - `extract_64`
  - `extract_256`
  - `decode_tag`
  - `texture_for_hit_128` (a full store)
- [ ] **Step 2: Run** `mise run bench`. Record the medians in the ledger. Expected: `decode_tag` and `texture_for_hit_128` take less than 100 ns each. If not, report to the operator before Task 8.
- [ ] **Step 3: Checkpoint.**

### Task 7: Game mirrors and skin and packet readers (amml crate)

**Files:**
- Create: `heads/amml/Cargo.toml` (cdylib + rlib, lib `alaydriem_locator_heads`, `autotests = false`), `heads/amml/src/lib.rs` (`#![deny(unsafe_code)]`)
- Create: `heads/amml/src/entries/{mod.rs, heads_entries.rs}`. `HeadsEntries` holds the Task 2 ids as `EntryId::new` constants.
- Create: `heads/amml/src/layouts/{mod.rs, client_waypoint_layout.rs, list_node_layout.rs, texture_ptr_layout.rs, image_layout.rs, image_description_layout.rs, resource_location_layout.rs}`
- Create: `heads/amml/src/structs/{mod.rs, packet_offsets.rs}`. It holds the R8, R9 and R10 offsets that are resolved from field entries at enable.
- Create: `heads/amml/src/services/{mod.rs, skin_reader.rs, packet_observer.rs}`
- Test: `heads/amml/tests/integration.rs`, `heads/amml/tests/layouts/*.rs` (offset asserts), `heads/amml/tests/services/{skin_reader.rs, packet_observer.rs}`

**Interfaces:**
- Consumes: the Task 1 table, the Task 2 ids, `FaceExtractor`, `FaceUpdate`.
- Produces:
  - `ClientWaypointLayout`: offsets 0x08 pos, 0x18 `optional<Color>` (flag 0x28), 0x30 `optional<string>` (flag 0x50), 0x58 icon size, 0x60 visible. Entity actor id at 0x78; the entity vtable is checked through `ClientWaypointLayout::actor_id(ptr, entity_vtable) -> Option<i64>`.
  - `SkinReader::view(skin_ref: usize, offsets: &PacketOffsets) -> Option<SkinView<'static>>`.
  - `PacketObserver::new(sender: crossbeam_channel::Sender<FaceUpdate>, offsets: PacketOffsets)`.
  - `fn on_player_list(&self, packet: usize)`.
  - `fn on_player_skin(&self, packet: usize)`.
  - Both use `try_send`. When the channel is full they count and drop. Log the drop once per 256 drops.

- [ ] **Step 1: Write failing tests.** They build fake game memory from `#[repr(C)]` test structs with the 26.60 offsets, and pass the address.
  - `reads_rgba_skin_through_ref`
  - `null_ref_gives_none`
  - `image_size_mismatch_gives_none` (blob size is less than w*h*4)
  - `player_list_add_sends_add_update`
  - `player_list_remove_sends_remove`
  - `unknown_variant_index_is_skipped`
  - `bad_skin_in_add_sends_nothing`
  - `player_skin_sends_skin_update`
  - `full_channel_drops_without_blocking`
  - Layout tests: `offset_of!` equals the Task 1 values.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.** Every pointer read checks for null first. Use `amml-msvc` `MsvcString`/`Optional` and the vector layouts. The `unsafe` is local, with `// SAFETY:` naming the detour that guarantees the packet lives for the call.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

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

### Task 9: Mod glue, build gate, manager link and packaging

**Files:**
- Create: `heads/amml/src/heads_mod.rs` (`LocatorHeadsMod: AmmlMod`, `NAME = "alaydriem-locator-heads"`), `heads/amml/src/services/{build_gate.rs, hook_installer.rs, heads_link.rs, descriptor.rs}`, `heads/amml/src/detours/{player_list.rs, player_skin.rs, client_update.rs, exit_level.rs}`
- Create: `heads/amml/amml.hcl`. `entries` lists every Task 2 id plus `ClientInstance.update`, `ClientInstance.getLocalPlayer`, `ClientExitLevel.hook`, `LegacyClientNetworkHandler.vtable` and `Mesh.size`.
- Create: `heads/amml/descriptor.hcl`. It has `claim "Hud" { mode = "Shared", reason = "Replaces the dots on the vanilla locator bar with player faces" }`, and a section `heads` with the fields `heads.enabled` and `heads.scale` (values from Global Constraints, `apply = "Live"`).
- Modify: `mise.toml`, adding the tasks `build-mod` and `package` (dist `dist/alaydriem-locator-heads/`: DLL, PDB, `amml.hcl`).
- Test: `heads/amml/tests/heads_mod.rs`, `heads/amml/tests/services/{build_gate.rs, heads_link.rs}`, `heads/amml/tests/support/{fake_host.rs, mod.rs}` (the zoom fake-host pattern)

**Interfaces:**
- Consumes: everything above. Copy the zoom patterns: `HookInstaller`, linking on the first `ClientInstance::update` (`zoom_runtime.rs:60-75`), and `ManagerLink::without_frame`.
- Produces: the DLL with exactly the four `amml_mod_*` exports.

- [ ] **Step 1: Write failing tests** with a fake host.
  - `refuses_when_mesh_size_is_not_2660`: `Mesh.size` = 0x278. No hook is installed, and one log line has the field `mesh_size`.
  - `refuses_when_an_entry_is_missing`
  - `installs_six_hooks_on_2660`: render, drawIcon, PlayerList, PlayerSkin, ClientInstance.update and ClientExitLevel.
  - `exit_level_clears_store_and_unloads_textures`
  - `settings_change_applies_live`
  - `runs_with_defaults_when_manager_missing`
  - `descriptor_parses_and_has_two_fields`: through `amm-core` descriptor checking, not a serde round trip.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.**
  - `enable` checks the gate, then installs the hooks. The vslot hook targets are read from the vtable globals, as stratum's `table_slot` does.
  - Create the channel (capacity 256) and the `RenderState` (store capacity 128, budget 4).
  - `disable` removes the hooks first, then drops the state.
- [ ] **Step 4: Run** `mise run test`, `mise run check` and `mise run build-mod`. Expected: PASS, clean, and `dist/alaydriem-locator-heads/` with 3 files.
- [ ] **Step 5: Checkpoint.**

### Task 10: In-game verification on 26.60.05 (end-to-end)

**Files:**
- Modify: `amml/scripts/ingame-mods.sh`, adding the line `../locator-heads:package:dist/alaydriem-locator-heads:alaydriem-locator-heads:alaydriem_locator_heads.dll:target/release/alaydriem_locator_heads.d`.
- Create: `amml/scripts/ingame-standalone-locator-heads.sh` and the `ingame-standalone-locator-heads` task in `amml/mise.toml` (a copy of `ingame-standalone-freecam` with `--game` and `--instance`).
- Create: `docs/findings/ingame-2660/findings.md`

- [ ] **Step 1: Add a one-time debug log** (Curia, behind the `trace` level): the thread id the first time each detour runs (R12), the number of faces uploaded, and the number of tag decode misses.
- [ ] **Step 2: Run** `mise run ingame-standalone-locator-heads -- --game <1.26.60.05> --instance <1.26.60.05-locator-heads>`. Before the first run, make the private instance as a copy of `1.26.60.05-standalone`, but not its `mods/amml` folder. Copy only the mod manager into it, then this mod. The script must stop only the PID in `locator-heads.pid`; change the copied script where it does otherwise. Expected:
  - `resolved N/N`
  - `"missing":0`
  - `features off: none`
  - `mod loaded` for `alaydriem-locator-heads`
  - no panic, ERROR or WARN lines
- [ ] **Step 3: Manual check (operator).** The operator joins a world or server with at least 2 other players (one with a classic 64x64 skin, one with a persona skin), with the locator bar on. Take screenshots with `amml/scripts/capture-game-window.ps1`:
  - (a) faces on the bar
  - (b) a player walking behind you: the arrows and fade stay vanilla
  - (c) a skin change: the face updates
  - (d) a player leaves: the icon goes, with no crash
  - (e) `heads.scale` 2.0 in the manager panel
  - (f) world exit and rejoin
  - (g) `heads.enabled` off: vanilla dots
- [ ] **Step 4: Record** the results, the R12 thread ids and the screenshots in `docs/findings/ingame-2660/findings.md`. End the file with "Task Completed". Send it with SendUserFile.
- [ ] **Step 5: Run the end-to-end suites:**
  - `mise run game-test` in `amml/`
  - `mise run e2e` in `modmanager-amml/`
  - the full `mise run test` in all three repos

  Expected: all pass. Report any failure with its output; do not hide it.
- [ ] **Step 6: Checkpoint.** If the work ran in a worktree, mirror `docs/superpowers/` and `docs/findings/` to the root branch under the same paths.
