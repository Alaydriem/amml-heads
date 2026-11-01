# Chat Heads Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Show each player's skin face as a font glyph just before their name in vanilla chat lines (chat, whisper, `/me`, join and leave), in the HUD chat and the chat window, inside `alaydriem-locator-heads`.

**Architecture:**
- The mod owns the glyph page `font/glyph_E1`: vanilla row 0, faces in rows 1 to 15 (U+E110 to U+E1FF). A CPU copy is kept in core and uploaded with the existing `TextureGroup::uploadTexture` path, at most once per frame, from the `ClientInstance::update` detour.
- The packet side gives each player a fixed cell when the face is extracted, and records the player's XUID and name.
- A post-detour on the `GuiMessage` constructor asks core where the glyph goes and writes the new full string back. A detour on `BitmapFont::_getCharWidth` returns 16.0 for cells that hold a face.

**Tech Stack:** Rust 2024, `amml-sdk`, `amml-msvc`, `amml-game` (path `../amml`), `amm-client`/`amm-core` (path `../modmanager-amml`), Curia, Criterion, cargo-nextest, mise.

**Spec:** `docs/superpowers/specs/2026-10-31-chat-heads-design.md`. Evidence: `docs/findings/chat-heads-spike/findings.md`, `docs/findings/chat-heads-builder-check/report.md`. Read all three before starting.

## Global Constraints

- Target build: 1.26.60.05, BuildId `6ad79ee4-17d51891a1119914`. Test only in the private instance `%LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads`. Write each game PID you start to `<instance>\locator-heads.pid` and end only that PID. Never use `taskkill /IM`. Never write to `%APPDATA%\levilauncher.exe\versions\`. Do not analyse LeviMC closed binaries.
- **Review policy (operator rule):** no per-task reviewer. Each task's own tests are the gate. One whole-branch review after the last task.
- **No git commits** (operator rule). Each "Checkpoint" step means: run the listed commands, record the result in `docs/findings/plan-ledger-chat/task-N.md`, and end the file with "Task Completed".
- All rules in `CLAUDE.md` apply: logic in `locator-heads-core` (`#![forbid(unsafe_code)]`), thin detours, one struct per file, `new()` constructors, no free functions, no `unwrap`/`expect` outside tests, `// SAFETY:` on every `unsafe` block, comments on the preceding line with no plan references, tests mirror `src/` in the one `integration` target, no zero-value tests, Curia logging with fields.
- Build and test only through mise tasks (`mise run check`, `mise run test`, `mise run package`). Never pipe build, test, bench or derive output into `grep`, `head`, `tail` or `awk`.
- Recipes follow `amml/docs/recipes.md`: two independent recipes and a check, wildcarded immediates, displacements and branch bytes, `notes` on each entry, `mise run lint` 0 open, `derive` unchanged for every existing entry.
- A failed entry or hook for chat heads turns off chat heads only. The locator bar keeps working, and the log says `features off: chat heads` with the reason.
- Fixed values:

  | Value | Setting |
  | --- | --- |
  | Glyph page | `font/glyph_E1`, 256x256 RGBA8, file system 0 |
  | Face cells | rows 1 to 15, U+E110 to U+E1FF, 240 cells, cell `i` at x = (i % 16) * 16, y = (i / 16 + 1) * 16 |
  | Cell face | the 32x32 face scaled down 2x (nearest, sample (2x, 2y)) to 16x16 |
  | Glyph width | 16.0 |
  | Cell bound | 240 for the life of the process, no eviction |
  | Page uploads | at most 1 per frame |
  | `heads.chat` | bool, default true, apply Live |
  | `GuiMessage::MessageType` | Chat 1, Localized 3, Whisper 5 |

## Review Focus

1. **A name inside a longer name** (`Steve` and `Steve2` are both players; a join line names `Steve2`). Only `Steve2` matches. Task 6: `a_name_inside_a_longer_name_is_not_matched`.
2. **Formatting codes before the name** (`§eSteve joined the game`). The glyph goes after `§e`, before `S`, so the glyph is not coloured away from the name and the code still applies to the name. Task 6: `the_glyph_goes_after_the_formatting_codes_before_the_name`.
3. **Two players with one name** (a departed `Alex` and a new `Alex` with another UUID). No head. Task 5: `two_players_with_one_name_give_no_match`.
4. **Names that are not ASCII** (`Zoë`). Byte offsets stay on character boundaries, and the match works. Task 6: `a_name_that_is_not_ascii_is_found_on_a_character_boundary`.
5. **The 241st player.** No cell, no head, one warn line per process. Task 4: `cells_stop_at_240_and_never_move`.

---

### Task 1: Confirm the open reverse-engineering facts (docs only, no product code)

**Files:**
- Create: `docs/findings/chat-heads-re/findings.md` (end with "Task Completed").

Use `mise run inspect` and `mise run decompile` in `../amml` on the 26.60.05 exe. Tag each fact [proven] (code evidence) or [inference].

- [ ] **Step 1: `GuiMessage` constructor.** Find the RVA of `GuiMessage(MessageType, std::string const& username, std::string const& message, std::optional<std::string> filteredMessage, std::string const& ttsMessage, std::string const& authorXuid, float, bool, bool)` (start from the callers of `GuiData::_addMessage` and `displayChatMessage`). Record:
  - its registers and stack arguments;
  - whether it builds `mFullString` and `mFilteredFullString` itself;
  - the field offsets it writes. Header values to confirm: `mType` 0x00, `mMessage` 0x08, `mFilteredMessage` 0x28, `mTTSMessage` 0x50, `mUsername` 0x70, `mFullString` 0x90, `mFilteredFullString` 0xB0, `mAuthorXuid` 0xD8.
- [ ] **Step 2: Every path that makes a chat line.** List the callers of the constructor and any other code that writes `mFullString` (a copy constructor, `_addMessage(MessageConfig)` building inline). Say whether the chat window (the screen opened with T) shows `mFullString` or builds its text from `mUsername` and `mMessage`. If the constructor is not the only path, name the insertion point that covers both the HUD and the chat window (spec Decision 5 allows this).
- [ ] **Step 3: Text forms.** From the 26.60 `en_US.lang` and the constructor, record the exact full string of a chat line, a whisper, `/me` (`chat.type.emote`), join (`multiplayer.player.joined`) and leave (`multiplayer.player.left`). Record which `MessageType` each arrives with.
- [ ] **Step 4: Player list Add entry.** Find the offsets of `mName` and `mXUID` (each a `std::string`) in the 26.60 add entry, with the existing entries `PlayerListAddEntry.size` and `.mSkin` as anchors (header order: `mId`, `mUUID`, `mName`, `mXUID`, ...).
- [ ] **Step 5: Font.** Confirm the `BitmapFont` vtable RVA `0xf0ea8a0`, slot 29 `_getCharWidth` (`0x15902c0`, `rcx` = font, `edx` = character, `r8b` = force unicode, result in `xmm0`). Confirm that the chat font is the `FontHandle` default font (+0x30) from `ClientInstance.getFontHandle`, and record which font class the smooth font option uses.
- [ ] **Step 6: An E1 icon before a world.** Search the vanilla UI JSON (`data/resource_packs/vanilla/ui`) for U+E100 to U+E10F. Name one screen outside a world that shows an E1 icon. Task 9 uses it.
- [ ] **Step 7: Checkpoint.** Ledger `task-1.md` lists each fact and its tag.

### Task 2: Recipes for the new entries (`../amml/recipes`)

**Files:**
- Create or modify in `../amml/recipes`: `gui_message.hcl` (new), `font.hcl` (add), `player_list_packet.hcl` (add).
- Modify: `heads/amml/src/entries/heads_entries.rs`, `heads/amml/amml.hcl`.

**Interfaces:**
- Produces entry ids: `GuiMessage.ctor` (function, `hooked = true`; or the Task 1 Step 2 insertion point), `BitmapFont.vtable`, `BitmapFont._getCharWidth` (function, `hooked = true`), `PlayerListAddEntry.mName`, `PlayerListAddEntry.mXUID` (fields), `ClientInstance.getFontHandle` (existing vslot, add to the manifest).
- Produces `HeadsEntries::CHAT: [EntryId; 6]`, which is also appended to `MANIFEST` and `amml.hcl`.

- [ ] **Step 1: Write each entry** with `mise run inspect -- draft <rva>`, wildcard it, check that `inspect scan` gives one match, and add two independent recipes, a check and `notes`.
- [ ] **Step 2: Run** `mise run derive -- --exe <26.60 exe> --recipes recipes --out <tmp> --report <tmp> --compare data/6ad79ee4-17d51891a1119914.hcl` twice. Expected: each new entry resolves to its Task 1 value, no existing value moves, and both runs are identical.
- [ ] **Step 3: Run** `mise run lint -- --exe <26.60 exe> --recipes recipes --report <file>`. Expected: 0 open.
- [ ] **Step 4: Update** `HeadsEntries` (`CHAT`, `MANIFEST` length) and `amml.hcl`. The existing test `manifest_lists_every_entry_the_mod_resolves` must pass. Update `entry_ids_are_distinct` to cover `CHAT`.
- [ ] **Step 5: Checkpoint.** `mise run check` and `mise run test` in `../amml` and here pass. Copy the changed recipe files into the private instance `amml/recipes`.

### Task 3: The `heads.chat` setting

**Files:**
- Modify: `heads/core/src/config/heads_settings.rs`, `heads/amml/descriptor.hcl`.
- Test: `heads/core/tests/config/heads_settings.rs`, `heads/amml/tests/services/descriptor.rs`.

**Interfaces:**
- Produces: `HeadsSettings { enabled: bool, scale: f32, chat: bool }`, `HeadsSettings::new(enabled: bool, scale: f32, chat: bool) -> Self`, serde default `HeadsSettings::default_chat() -> bool` (true).

- [ ] **Step 1: Write the failing tests.** In `settings_default_and_clamp`, assert `HeadsSettings::default().chat == true`, and that a document without `chat` reads as `true`. In `descriptor_parses_and_lists_its_fields`, expect `["heads.enabled", "heads.scale", "heads.chat"]`. `schema_defaults_match_core_defaults` covers the default.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Add the field** and the descriptor field `heads.chat` (label "Show faces in chat", help "Draws each player's face before their name in chat", kind bool, default true, apply Live). Fix every `HeadsSettings::new` caller.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

### Task 4: Glyph cells (core)

**Files:**
- Create: `heads/core/src/services/glyph_cells.rs`, `heads/core/src/services/cell_flags.rs`, `heads/core/src/structs/chat/glyph_cell.rs`, `heads/core/src/structs/chat/mod.rs`.
- Test: `heads/core/tests/services/glyph_cells.rs`.

**Interfaces:**
- Produces:
  - `GlyphCell(u8)` with `GlyphCell::codepoint(self) -> char` (U+E110 + index) and `GlyphCell::from_codepoint(c: u32) -> Option<GlyphCell>`.
  - `CellFlags::new() -> Self` (in `heads/core/src/services/cell_flags.rs`): 4 `AtomicU64` words. `CellFlags::set(&self, cell: GlyphCell)` and `CellFlags::has_face(&self, codepoint: u32) -> bool`, both `Ordering::Relaxed`. `has_face` does not lock and does not allocate, and returns false outside U+E110..=U+E1FF.
  - `GlyphCells::new(flags: Arc<CellFlags>) -> Self` with `CAPACITY: usize = 240`. The `Arc` lets the width detour read the flags without the chat state lock.
  - `GlyphCells::cell_for(&mut self, uuid: [u8; 16]) -> Option<GlyphCell>`: gives the next free cell on first sight, the same cell afterwards, `None` when full.
  - `GlyphCells::cell_of(&self, uuid: [u8; 16]) -> Option<GlyphCell>` (no assignment).
  - `GlyphCells::mark_face(&self, cell: GlyphCell)` (sets the flag) and `GlyphCells::has_face(&self, codepoint: u32) -> bool` (reads it).
  - `GlyphCells::full_reported(&mut self) -> bool`: true once, the first time `cell_for` returned `None`, so the caller warns once.

- [ ] **Step 1: Write the failing tests:**
  - `cells_stop_at_240_and_never_move`: 240 distinct UUIDs get cells 0..240 in order. The 241st gets `None`. Asking again for UUID 0 gives cell 0. `full_reported` is true once, then false.
  - `a_cell_has_a_face_only_after_mark_face`: `has_face(0xE110)` is false after `cell_for`, and true after `mark_face`.
  - `has_face_is_false_outside_the_face_range`: `0xE10F`, `0xE200`, `'A' as u32` give false even when every cell has a face.
  - `codepoint_round_trips_for_every_cell`.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.** The map is a `Vec<[u8; 16]>` with capacity 240 (linear search is fine at 240, and it runs at packet rate only). State the bound and "no eviction" in the type doc.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Bench.** Add `has_face_hit` and `has_face_miss_ascii` to `heads/bench/benches/heads.rs`. Record both numbers in the ledger.
- [ ] **Step 6: Checkpoint.**

### Task 5: Player directory (core)

**Files:**
- Create: `heads/core/src/services/player_directory.rs`.
- Test: `heads/core/tests/services/player_directory.rs`.

**Interfaces:**
- Produces:
  - `PlayerDirectory::new() -> Self`, `CAPACITY: usize = 240`.
  - `PlayerDirectory::record(&mut self, uuid: [u8; 16], xuid: &str, name: &str) -> bool`: adds or updates the player. False (nothing stored) when the directory is full and the UUID is new.
  - `PlayerDirectory::by_xuid(&self, xuid: &str) -> Option<[u8; 16]>`: `None` for an empty XUID.
  - `PlayerDirectory::by_name(&self, name: &str) -> Option<[u8; 16]>`: `Some` only when exactly one entry has that name (exact, case sensitive).
  - `PlayerDirectory::names(&self) -> impl Iterator<Item = (&str, [u8; 16])>`.
  - A Remove never deletes an entry. Only players with a cell are recorded (the caller records after `cell_for` gives `Some`), so the bound matches the cell bound.

- [ ] **Step 1: Write the failing tests:**
  - `a_player_is_found_by_xuid_and_by_name`.
  - `an_empty_xuid_finds_nothing`.
  - `two_players_with_one_name_give_no_match`: UUID A and B both named `Alex`, so `by_name("Alex")` is `None`; `by_xuid` still finds each.
  - `a_new_name_for_the_same_player_replaces_the_old_one`.
  - `the_directory_stops_at_240`: the 241st new UUID returns false and is not found; an update of a known UUID still works.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement** with a `Vec` of entries (capacity 240) holding owned `String`s.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

### Task 6: Chat line placement (core)

**Files:**
- Create: `heads/core/src/services/chat_head_placer.rs`, `heads/core/src/structs/chat/message_kind.rs`, `heads/core/src/structs/chat/chat_line.rs`.
- Test: `heads/core/tests/services/chat_head_placer.rs`.

**Interfaces:**
- Consumes: `GlyphCells` (Task 4), `PlayerDirectory` (Task 5).
- Produces:
  - `MessageKind { Chat, Whisper, Localized, Other }` with `MessageKind::from_raw(raw: i32) -> MessageKind` (1, 5, 3, anything else).
  - `ChatLine<'a> { kind: MessageKind, full: &'a str, username: &'a str, author_xuid: &'a str }`.
  - `ChatHeadPlacer::place(line: &ChatLine, cells: &GlyphCells, directory: &PlayerDirectory) -> Option<usize>`: the byte offset in `full` where the glyph goes, or `None` for no head.
  - `ChatHeadPlacer::insert(full: &str, at: usize, glyph: char) -> String`.
  - `ChatHeadPlacer::glyph_for(line: &ChatLine, cells: &GlyphCells, directory: &PlayerDirectory) -> Option<char>`: the cell codepoint of the player the line belongs to, only when that cell has a face.

Rules (spec Decision 6):
- Chat and Whisper: the player is `by_xuid(author_xuid)`, or `by_name(username)` when the XUID is empty. The offset is 0.
- Localized: for each directory name that occurs exactly once in `full` at a word boundary (the bytes before and after are not ASCII alphanumeric or `_`), the player is a candidate. Exactly one candidate player is required, and `by_name` must also be unique. The offset is the start of the name. Formatting codes (`§` plus one character) directly before the name stay before the glyph.
- Other: `None`. A player whose cell has no face: `None`.

- [ ] **Step 1: Write the failing tests:**
  - `a_chat_line_gets_the_glyph_at_the_start` (`<Steve> hi` gives offset 0 and Steve's codepoint).
  - `a_whisper_gets_the_glyph_at_the_start`.
  - `an_empty_xuid_falls_back_to_the_username`.
  - `a_join_line_gets_the_glyph_before_the_name` (`Steve joined the game` gives offset 0; `* Steve waves` gives offset 2).
  - `the_glyph_goes_after_the_formatting_codes_before_the_name` (`§eSteve joined the game`: the offset is the byte after `§e`).
  - `a_name_inside_a_longer_name_is_not_matched`.
  - `a_line_that_names_two_players_gets_no_head`.
  - `a_name_that_is_not_ascii_is_found_on_a_character_boundary` (`§eZoë joined the game`; `insert` gives valid UTF-8 with the glyph before `Z`).
  - `a_player_without_a_face_gets_no_head`.
  - `other_message_kinds_get_no_head` (`MessageKind::from_raw(4)`).
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.** `place` does not allocate. `insert` allocates once (once per chat line, not per frame).
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Bench** `place_chat_line` and `place_join_line_240_players` (a full directory). Record both numbers.
- [ ] **Step 6: Checkpoint.**

### Task 7: Glyph page (core)

**Files:**
- Create: `heads/core/src/services/glyph_page.rs`, `heads/amml/assets/glyph_e1_row0.rgba` (16384 bytes: row 0 of the vanilla 256x256 `glyph_E1`, 16 pixel rows, RGBA8).
- Test: `heads/core/tests/services/glyph_page.rs`.

**Interfaces:**
- Consumes: `GlyphCell` (Task 4), `FaceImage`.
- Produces:
  - `GlyphPage::new(row0: &[u8]) -> Self` (row 0 copied in; rows 1 to 15 transparent; dirty = true so the first frame uploads; a `row0` of the wrong length leaves row 0 transparent).
  - `GlyphPage::SIDE: u32 = 256`.
  - `GlyphPage::write_face(&mut self, cell: GlyphCell, face: &FaceImage)` (sets dirty).
  - `GlyphPage::take_dirty(&mut self) -> Option<&[u8]>`: the whole page once after a change, then `None` until the next change.

- [ ] **Step 1: Write the failing tests:**
  - `a_new_page_keeps_row_0_and_is_dirty_once`.
  - `a_face_lands_in_its_cell_scaled_down_2x`: a face whose pixel (2x, 2y) is unique gives page pixel (cell x + x, cell y + y) equal to it, for cells 0, 15, 16 and 239.
  - `writing_a_face_does_not_touch_row_0_or_other_cells`.
  - `take_dirty_returns_the_page_once_per_change`.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.** Make `glyph_e1_row0.rgba` from the vanilla page (extract `glyph_E1.png` from `data/resource_packs/vanilla/__brarchive/font.brarchive` of the 26.60 instance, take rows 0 to 15) and record how in the ledger.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

### Task 8: Chat state, page upload and packet feed (amml)

**Files:**
- Create: `heads/amml/src/services/chat_state.rs`, `heads/amml/src/services/chat_runtime.rs`, `heads/amml/src/layouts/player_list_entry_names.rs` (if Task 1 needs a mirror).
- Modify: `heads/amml/src/services/texture_bridge.rs`, `heads/amml/src/services/packet_observer.rs`, `heads/amml/src/structs/packet_offsets.rs`, `heads/amml/src/detours/client_update.rs`, `heads/amml/src/detours/exit_level.rs` (no cell clear: confirm only).
- Test: `heads/amml/tests/services/chat_state.rs`, `heads/amml/tests/services/texture_bridge.rs`, `heads/amml/tests/services/packet_observer.rs`.

**Interfaces:**
- Consumes: Tasks 4 to 7; entries `PlayerListAddEntry.mName`, `.mXUID` (Task 2).
- Produces:
  - `ChatState::new(page: GlyphPage, flags: Arc<CellFlags>) -> Self`, holding `GlyphCells`, `PlayerDirectory`, `GlyphPage`, and the settings flag `chat: bool`.
  - `ChatState::on_face(&mut self, uuid: [u8; 16], xuid: &str, name: &str, face: &FaceImage)`: `cell_for`, `record`, `write_face`, `mark_face`; warns once when cells run out.
  - `ChatState::on_skin(&mut self, uuid: [u8; 16], face: &FaceImage)`: rewrites the cell only if the player has one.
  - `ChatState::take_page(&mut self) -> Option<&[u8]>`.
  - `ChatState::glyph_line(&self, line: &ChatLine) -> Option<String>`: the new full string, or `None`. `None` when `chat` is off.
  - `ChatRuntime` (a static `Mutex<Option<ChatState>>`, like `RenderRuntime`, plus a static `OnceLock<Arc<CellFlags>>`): `install(state: ChatState, flags: Arc<CellFlags>)`, `uninstall() -> Option<ChatState>`, `with_state(f) -> Option<R>` using `try_lock` (a busy state gives `None`, never a block), and `has_face(codepoint: u32) -> bool`, which reads the `CellFlags` without the mutex and is false before install.
  - Logging: one info line `chat glyph page uploaded` the first time the page uploads; one warn line `chat head cells are full` (from `full_reported`).
  - `TextureBridge::upload_page(&mut self, group: usize, pixels: &[u8]) -> bool`: uploads 256x256 RGBA under one `ResourceLocation` `font/glyph_E1`, built once and freed in `shutdown`. It shares the pixel upload with `upload` (generalize `upload_pixels` to take `side` and `&[u8]`).
  - `PacketOffsets` gains `add_entry_name: usize` and `add_entry_xuid: usize`.
- Behaviour:
  - `PacketObserver` on an Add with a face: reads name and XUID (`StdStringReader`, bounded length) and calls `ChatRuntime::with_state(|s| s.on_face(..))`. On a Skin packet with a face: `on_skin`. This happens on the packet thread, before the face goes into the channel.
  - `client_update`: each frame, `ChatRuntime::with_state` takes the page and uploads it with the texture group (the frame's group is read the same way `RenderState::group_address` does). The first frame uploads the page with empty face rows (spec Decision 8). A frame with no change does nothing.

- [ ] **Step 1: Write the failing tests:**
  - `a_face_gets_a_cell_and_dirties_the_page_once` (`chat_state`).
  - `a_skin_change_rewrites_the_same_cell`.
  - `a_skin_for_a_player_without_a_cell_does_nothing`.
  - `a_line_gets_no_glyph_while_the_setting_is_off`.
  - `upload_page_builds_one_location_and_uploads_256_square` (`texture_bridge`, with the existing fake texture functions).
  - `player_list_add_feeds_the_chat_state_with_name_and_xuid` (`packet_observer`, using `tests/support/fake_skin.rs` extended with name and XUID strings).
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.** No allocation on the frame path when nothing changed. The upload runs at most once per frame.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Checkpoint.**

### Task 9: The two detours, the font check and the optional hook group (amml)

**Files:**
- Create: `heads/amml/src/detours/gui_message.rs`, `heads/amml/src/detours/char_width.rs`, `heads/amml/src/layouts/gui_message_layout.rs`, `heads/amml/src/services/chat_font_check.rs`.
- Modify: `heads/amml/src/heads_mod.rs`, `heads/amml/src/services/hook_installer.rs`, `heads/amml/src/detours/mod.rs`.
- Test: `heads/amml/tests/detours/gui_message.rs`, `heads/amml/tests/detours/char_width.rs`, `heads/amml/tests/layouts/gui_message_layout.rs`, `heads/amml/tests/heads_mod.rs`.

**Interfaces:**
- Consumes: `ChatRuntime` (Task 8), entries (Task 2), offsets (Task 1).
- Produces:
  - `GuiMessageLayout` constants (Task 1 Step 1 values) with an `offsets_match_2660` test.
  - `char_width` detour (`unsafe extern "C" fn(font: usize, character: i32, force_unicode: u8) -> f32`): returns `16.0` when `ChatRuntime::has_face(character as u32)`, otherwise the original. No lock, no allocation.
  - `gui_message` post-detour: runs the original, then (only when the chat font check passes) reads `mType`, `mFullString`, `mFilteredFullString`, `mUsername` and `mAuthorXuid`, calls `ChatState::glyph_line` for the full string and for the filtered string when it has a value, and writes each result back as a new `std::string` on the game heap (`MsvcHeap`), freeing the old buffer. A panic becomes a logged no-op (the original already ran).
  - `ChatFontCheck::is_bitmap(client: usize) -> bool`: the default font of `getFontHandle` has the `BitmapFont.vtable`. The FontHandle is destroyed with `FontHandle.dtor`.
  - `LocatorHeadsMod::chat_hooks() -> [HookSpec; 2]`, installed after `hooks()`. If a chat entry does not resolve or a chat hook fails, the mod logs `features off: chat heads` with the reason, removes any chat hook that went in, and keeps the locator bar hooks. Otherwise it logs `features off: none` as now.

- [ ] **Step 1: Write the failing tests:**
  - `a_chat_line_gets_the_glyph_written_into_both_strings` (`gui_message`, a fake `GuiMessage` buffer built with the layout).
  - `a_line_without_a_head_is_left_byte_for_byte`.
  - `a_missing_filtered_string_stays_missing`.
  - `width_is_16_for_a_cell_with_a_face_and_the_original_otherwise` (`char_width`).
  - `a_missing_chat_entry_turns_off_only_chat_heads` (`heads_mod`: the six existing hooks install, the log has `features off: chat heads`).
  - `offsets_match_2660` (`gui_message_layout`).
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.** Every detour is a thin delegator with `// SAFETY:` comments.
- [ ] **Step 4: Run** `mise run test` and `mise run check`. Expected: PASS and clean.
- [ ] **Step 5: Bench** the `char_width` decision through `ChatRuntime::has_face` (hit and miss). Record both numbers.
- [ ] **Step 6: Checkpoint.**

### Task 10: In-game verification

- [ ] **Step 1: Build and run.** `mise run package`, install into the private instance (DLL, PDB, `amml.hcl`; the recipes from Task 2 in `amml/recipes`), start `Minecraft.Windows.exe` directly and write the PID file.
- [ ] **Step 2: Log check.** The log has `features off: none`, all entries resolve, and the page upload line appears once.
- [ ] **Step 3: Operator checks** (ask the operator; each needs a screenshot):
  1. A chat line in a single-player world shows the face before `<Name>`, at full width.
  2. The chat window (T) shows the same head.
  3. A whisper (`/tell @s hi`) and `/me waves` show a head.
  4. With a second player on a server: their join line, chat line and leave line show their face; a skin change updates old lines.
  5. `heads.chat` off in the manager panel: new lines have no head; old lines still draw without overlap.
- [ ] **Step 4: Early load check** (spec Decision 8): restart, open the Task 1 Step 6 screen before joining a world, then join and send a chat line. If no head draws, record it as a known problem in `docs/findings/chat-heads-re/findings.md` and continue (operator decision).
- [ ] **Step 5: End only the PID you started.** Checkpoint with the results and screenshot paths.

### Task 11: Final review

- [ ] **Step 1: Run** `mise run check`, `mise run test` and the bench (`cargo bench -p locator-heads-bench` through its mise task if one exists). Record the numbers.
- [ ] **Step 2: Dispatch one whole-branch review** on the most capable model, with the spec, this plan and the diff. Fix what it confirms, then rerun Step 1.
- [ ] **Step 3: Update the spec status line** to say what shipped and list the known problems. Checkpoint.
