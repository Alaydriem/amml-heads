Task 8 report

Files changed
- heads/core/src/services/glyph_page.rs (mark_dirty), heads/core/tests/services/glyph_page.rs (+1 test)
- heads/amml/src/services/chat_state.rs, chat_runtime.rs (new), mod.rs
- heads/amml/src/services/texture_bridge.rs (upload_page, page location, upload_pixels takes side + pixels)
- heads/amml/src/services/render_state.rs (upload_chat_page), render_runtime.rs (upload_chat_page, try_lock)
- heads/amml/src/services/packet_observer.rs (chat feed), heads_runtime.rs (chat flag follows settings)
- heads/amml/src/structs/packet_offsets.rs (Option name/xuid offsets, new_with_chat; new() unchanged)
- heads/amml/src/detours/client_update.rs (ChatRuntime::flush_page(this))
- tests: chat_state, texture_bridge, packet_observer, support/fake_game, support/fake_packet
- exit_level.rs: confirmed, no cell clear.

Results
- mise run check: clean
- mise run test: 178 tests run: 178 passed

Concerns
- ChatRuntime flags are process-wide (OnceLock); a re-enable builds a fresh ChatState with ChatRuntime::flags(), old flag bits stay set.
- client_update passes `this` as the client for the texture group; confirm in game.
- Task 9 must build PacketOffsets with new_with_chat and install ChatState (ChatState::VANILLA_ROW0 is the embedded row 0).


Fix round
- ChatState lives for the process: ChatRuntime::install resumes the kept state (marks page dirty), ChatRuntime::suspend replaces uninstall. A suspended state ignores on_face, on_skin, flush_page and glyph_line.
- Packet feed uses ChatRuntime::with_state_blocking (blocking lock, packet feed only); game-thread paths keep try_lock.
- New tests (tests/services/chat_runtime.rs): a_re_enable_keeps_every_cell, a_re_enable_offers_the_page_for_upload_again.
- mise run check clean; mise run test: 180 tests run: 180 passed, 0 skipped.
- Task 9: disable must call ChatRuntime::suspend(), enable ChatRuntime::install(...).

Task Completed
