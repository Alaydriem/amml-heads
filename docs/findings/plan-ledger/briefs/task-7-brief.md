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

