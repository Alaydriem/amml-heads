Chat heads: final whole-branch review
=====================================

Scope: every file in docs/findings/plan-ledger-chat/changed-files.txt that holds code (amml src, core src, the
tests that cover the chat path, the recipes and the manifest). It also covers the amml_msvc types the edit uses
(MsvcString, Optional, MsvcHeap) and the amml_detour! macro.

Verification run by the reviewer:
- mise run test: 189 tests run, 189 passed, 0 skipped.
- mise run check: cargo check, clippy -D warnings and fmt --check are all clean.

Counts: critical 0, important 2, minor 5.

The areas below have no finding. Each was checked by reading the code:
- gui_message detour: it reads only `this`, after the original (gui_message.rs:26-41). The layout offsets match
  the findings and a test checks them (0x90, 0xB0, engaged byte 0xB0+0x20 = 0xD0, 0xD8, size 0x110).
  replace() makes the new string before it frees the old block. It frees each old block once, with
  capacity + 1, which is the size MSVC uses for both the SSO case and the big-allocation case. The full string
  and the filtered string are separate buffers. No double free and no leak per line.
- The filtered string is written only when its engaged byte is set (gui_message_layout.rs:56).
- char_width does no allocation and takes no lock: one OnceLock get and a range check (char_width.rs:10).
- Font check: the FontHandle is built in a local 0x50 byte buffer, its destructor runs once, and a null font
  gives false (chat_font_check.rs:41-49).
- Packet observer string reads are bounded to 64 bytes (packet_observer.rs:27, 220-221).
- Page upload: the ResourceLocation is built once and kept. TextureBridge::shutdown frees it with the slot names
  when the world is left. "font/glyph_E1" is 13 bytes, so it is an SSO string and nothing leaks on the game
  heap.
- Threading: every chat-lock use on the game thread per frame or per line is try_lock (flush_page,
  edit_message). The only nesting is CHAT -> RENDER, and the inner one is a try_lock
  (render_runtime.rs:84). No path holds RENDER and then waits for CHAT, so no deadlock is possible. If a GuiMessage
  is built while the same thread holds the chat lock (for example a log line shown in chat from inside
  feed_chat), the try_lock fails and that line gets no head. It does not deadlock.
- Optional group: chat_parts failing or the chat install failing keeps the 6 locator-bar hooks.
  HookInstaller::install removes only the hooks made since `before`. HOOK_CAPACITY is 8 and 6 + 2 = 8. disable
  removes all hooks, then suspends the chat state. A missing manifest entry is listed by the loader as missing
  and does not stop the mod from loading.
- Panics: amml_detour! catches them. After the original ran, the original's result is returned. No slice index
  in the placer or the page can go past its end (anchor/insert are on char boundaries; cell < 240; face_y max
  251 < 256).
- CLAUDE.md: one struct per file, no free functions outside detours and benches, no unwrap/expect in src, every
  unsafe block has a SAFETY comment, comments are on the preceding line with no plan references, each bound has
  a test (glyph_cells cells_stop_at_240_and_never_move, player_directory the_directory_stops_at_240), and there
  is a bench for has_face and for the placer.


IMPORTANT
---------

I-1. Any player can show another player's face by typing a face code point.
  Where: heads/core/src/services/chat_head_placer.rs:22-28 (insert) and 35-49 (resolve). Also
         heads/amml/src/detours/char_width.rs:10.
  What: The cells are the fixed code points U+E110..U+E1FF. Neither the placer nor the edit checks the line for
  these characters. The width detour widens a face cell wherever it appears, and the page draws the face there.
  Scenario: Bob types (or pastes) U+E110 in chat. The line becomes "[Bob face]<Bob> [Alex face]". The same
  works in a whisper, a /me text, a sign, a renamed item or an offline-server name. Vanilla Bedrock clients can
  paste E0xx/E1xx glyphs, and a vanilla server does not filter them. This is the first anti-goal: the line shows
  the head of a player who did not write it.
  Fix: in core, before inserting, replace every U+E110..U+E1FF character in the line text with nothing, or with
  a code point outside the range. ChatHeadPlacer::insert already copies the string, so this costs no extra
  allocation. Then put only the one glyph core chose. Do this also when the line gets no head: edit_message
  then writes the cleaned string, so it must also write when only cleaning was needed. Add a core test: a chat
  line whose message holds U+E110 comes out with only the author's glyph. Text outside chat (signs, nametags)
  is not covered by this. Write that down as a known limit, or cover it by making char_width/the page answer
  only for lines the mod edited (that needs more work).

I-2. When the current player is not recorded, a stale entry with the same name gives a wrong face.
  Where: heads/amml/src/services/packet_observer.rs:130-145 (feed_chat runs only after the face is extracted
         and the channel had room), heads/amml/src/services/chat_state.rs:52-67 (no record without a cell),
         heads/core/src/services/player_directory.rs:39-46 (by_name).
  What: The directory keeps names of departed players for the life of the process, across worlds and servers.
  A new Add entry is recorded only when (a) the channel is not full, (b) the face was extracted, and (c) a cell
  is free. If one of these fails for the player who is in the world now, the stale entry is the only one with
  that name, and by_name / resolve_named treat it as "exactly one match".
  Scenario: On server 1 the operator met "Alex" (UUID A), and the cell holds A's face. Later, on server 2 (or
  the same server after a gamertag change), a different player named "Alex" (UUID B) joins. B's skin cannot be
  read (SkinNotReadable, bad size), or the channel is full during a mass join, or all 240 cells are in use.
  "§eAlex joined the game", "* Alex waves", and any chat from B with an empty XUID (offline/LAN) all show A's
  face. This is the first anti-goal: when the mod is not sure, the line must get no head.
  Fix: record the identity (UUID, XUID, name) of every Add entry, even with no face and no cell, so that a name
  shared with a stale entry becomes ambiguous and gives no head. One way: keep a bounded name-only list
  (capacity 240, oldest evicted, with a bound test) that by_name/resolve_named also count as matches. A
  stricter option: name lookups count only entries seen since the last world join, and the cells still never
  move. Add a core test: a stale "Alex" plus a new unrecorded "Alex" gives no head.


MINOR
-----

M-1. by_xuid has no uniqueness check, and the XUID offset is still an inference.
  Where: heads/core/src/services/player_directory.rs:32-37; findings.md line 22 (mXUID +0x40 [inference]).
  Scenario: +0x40 may be a different string field that holds one same non-empty value for every player. A
  server that sends one placeholder XUID for all players gives the same result. by_xuid then returns the first
  player recorded, so every chat line shows that player's face. The ledger's in-game check ("chat lines get no
  head while join lines do") does not catch this with only one other player in the test.
  Fix: give by_xuid the same "exactly one match" rule as by_name. It costs one more iterator step.

M-2. Lines with a glyph keep the width-11 cell when the page is not on the font's texture. This leaves a blank
     gap, not a vanilla line.
  Where: heads/amml/src/services/chat_runtime.rs:115-136 (the edit depends only on the font class and the cell
         flags), heads/core/src/services/chat_state.rs:85-98.
  Scenario: Spec Decision 8 says that when the early-load upload fails, "lines look vanilla". The code still
  inserts the glyph and widens it, so the lines show an empty 11 px gap (third anti-goal). The same happens
  after a late enable from the manager after the font has loaded glyph_E1, which the spike showed does not
  draw. It may also happen if a resource reload (for example a world with packs) reloads the vanilla
  glyph_E1: nothing marks the chat page dirty after a world exit or a reload, so it is not uploaded again. This
  last case was not checked in game.
  Fix: insert glyphs only after at least one page upload has succeeded in this session (a flag set in
  ChatState::flush_page after upload() returns true). Also call page.mark_dirty() on the world-exit path
  (exit_level / Clear) so the page goes up again after a reload.

M-3. A page upload that keeps failing runs again every frame.
  Where: heads/core/src/services/chat_state.rs:89-92; heads/amml/src/services/render_state.rs:130-141.
  Scenario: The upload can fail because the ImageBuffer size does not match, the texture group is 0, or the
  render state is busy. Each failure marks the page dirty again. If the cause stays, every frame calls
  getTextureGroup, builds and frees an ImageBuffer of 256 KB, and releases the shared_ptr. That is work per
  frame with no change, against the second anti-goal.
  Fix: after N failures in a row, back off (for example retry every 64 frames, as the font check does), and
  log the first failure once.

M-4. No test proves that the old string block is freed exactly once.
  Where: heads/amml/tests/support/fake_heap.rs, heads/amml/tests/detours/gui_message.rs.
  What: FakeHeap goes straight to the Rust allocator and counts nothing. A leak of the old full string, or a
  free with the wrong size, would pass every test. The release path was correct when reviewed, but nothing
  stops a later change from breaking it.
  Fix: give FakeHeap atomic allocate/free counters (and record the size passed to free). Assert that one
  edited line with a heap-mode full string and filtered string frees 2 blocks with sizes capacity + 1.

M-5. The spec and the code give two different widths.
  Where: spec Decision 4 says 16.0. The code uses GlyphCell::WIDTH = 11.0 (glyph_cell.rs:10), as the ledger
  ruling decided in game.
  What: The code follows the ruling, which is correct. The spec text was not updated, so a later reader may
  "fix" the code back to 16.
  Fix: add a note to the spec, or a line under Decision 4, that the ruling changed the width to 11.0 with a 10x10
  face at y 2..11.

Task Completed
