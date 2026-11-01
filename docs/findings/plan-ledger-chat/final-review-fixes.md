Chat heads: fixes for the final review
======================================

Findings
--------
I-1 fixed. The core method ChatHeadPlacer::edit(line, cells, directory) -> Option<String> removes every U+E110..U+E1FF character and puts in the one chosen glyph. When the line gets no head, it gives the cleaned string. It gives None (no allocation) when the line holds no face code point and gets no head. insert() now also cleans. The new clean() first checks for the 0xEE lead byte, so a line without one does no char walk. ChatState::glyph_line uses edit() (or clean() before the first upload). It runs on the full string and on the filtered string. edit_message already writes whenever glyph_line gives Some, so it writes a line that only needed cleaning. A line with no face code point and no head stays byte for byte (the test passes).
I-2 fixed. The new core type RecentPlayers (services/recent_players.rs) holds UUID, XUID and name. Its bound is 240 and it evicts the oldest; noting a known UUID again updates it and makes it the newest. The bound is stated in the type doc. PlayerDirectory::note() puts a UUID with no cell into RecentPlayers, or updates the recorded entry when the UUID already has a cell. record() takes a UUID out of RecentPlayers, so each UUID is in one list only. by_name, by_xuid and names() (the anchored-name rule) count both lists. Recorded entries are never evicted, as before. ChatState::on_identity() calls note(). PacketObserver::on_entry now reads the name and XUID first (identity()). It feeds on_identity before drop_when_full and before the face extraction, and calls on_face after extraction as before. Both use with_state_blocking (the packet thread only, as before). The channel stays try_send / is_full.
M-1 fixed. by_xuid gives Some only when exactly one UUID across both lists has that XUID (the same sole() rule as by_name).
M-2 fixed. ChatState has page_uploaded, which is set when the upload returns true; it replaces page_logged and is never reset in the process. Before it is set, glyph_line only cleans and puts in no glyph. World exit: the exit_level detour calls ChatRuntime::on_world_exit(), which sets the atomic PAGE_STALE (SeqCst) and never waits. The next ChatRuntime::flush_page_with takes the flag inside the state try_lock and calls ChatState::on_world_exit(), which marks the page dirty and resets the back-off.
M-3 fixed. The new core type UploadBackoff (services/upload_backoff.rs): failures 1 and 2 retry on the next frame. From the 3rd failure in a row, it waits 63 frames, so a retry runs every 64 frames. reset() on success. failed() is true only for the first failure in the life of the back-off. ChatState::flush_page logs that one with curia::warn!("chat glyph page upload failed", { failures_before_backoff, retry_frames }).
M-4 fixed. FakeHeap has static atomic allocate and free counters and keeps a list of the sizes passed to free. The new test an_edited_line_frees_each_old_heap_string_once_with_its_size asserts 2 allocations, 2 frees, and sizes [full.len()+1, filtered.len()+1] for 35-byte and 21-byte heap-mode strings. new_in gives capacity = len, so these sizes are capacity + 1.
Follow-up (lead ruling) fixed. ChatState::glyph_line now cleans the line whatever the `chat` setting is, while the state is installed and not suspended. With `chat` off it gives ChatHeadPlacer::clean (no new glyph). Only a suspended state gives None. Test: a_typed_face_code_point_is_removed_while_the_setting_is_off. It checks that a line is cleaned with chat off, that a clean line gives None, and that a suspended state leaves the line alone.
M-5 not in scope (spec text). The face size is not changed (10x10, y 2..11, width 11.0).

Files changed
-------------
heads/core/src/services/chat_head_placer.rs   (edit, clean, insert cleans)
heads/core/src/services/player_directory.rs   (note, RecentPlayers, sole rule for name and XUID)
heads/core/src/services/recent_players.rs     (new)
heads/core/src/services/upload_backoff.rs     (new)
heads/core/src/services/mod.rs
heads/amml/src/services/chat_state.rs         (on_identity, on_world_exit, page_uploaded, backoff, glyph_line)
heads/amml/src/services/chat_runtime.rs       (PAGE_STALE, on_world_exit, flush_page_with)
heads/amml/src/services/packet_observer.rs    (identity before the channel and extraction checks)
heads/amml/src/detours/exit_level.rs          (calls ChatRuntime::on_world_exit)
heads/bench/benches/heads.rs                  (new bench place_edit_chat_line)
tests: heads/core/tests/services/{chat_head_placer,player_directory,recent_players,upload_backoff,mod}.rs,
       heads/amml/tests/services/{chat_state,chat_runtime,packet_observer}.rs,
       heads/amml/tests/detours/{gui_message,exit_level}.rs, heads/amml/tests/support/fake_heap.rs

New tests
---------
core: a_face_code_point_typed_into_a_line_is_removed, a_line_without_a_head_is_cleaned_of_face_code_points (U+E111),
      a_line_without_a_head_or_a_face_code_point_is_not_edited,
      a_stale_name_and_a_new_unrecorded_player_with_that_name_give_no_head,
      a_xuid_shared_with_an_unrecorded_player_gives_no_head, a_xuid_shared_by_two_players_gives_no_match (M-1),
      a_noted_player_makes_a_shared_name_and_xuid_ambiguous, a_noted_player_alone_is_found_by_name_and_xuid,
      noting_a_recorded_player_updates_it_and_adds_no_entry, recording_a_noted_player_keeps_one_entry,
      the_list_stops_at_240_and_evicts_the_oldest (bound), noting_a_known_player_again_updates_it_and_makes_it_the_newest,
      a_removed_player_is_gone, upload_backoff x4.
amml: no_glyph_is_put_in_before_the_page_has_uploaded, a_face_code_point_is_removed_before_the_page_has_uploaded,
      a_failing_upload_backs_off_after_three_failures, leaving_the_world_offers_the_page_again_at_once,
      a_player_seen_without_a_face_makes_a_shared_name_give_no_head,
      leaving_the_world_offers_the_page_again_on_the_next_flush (ChatRuntime),
      leaving_the_world_offers_the_chat_page_for_upload_again (exit_level detour),
      an_add_whose_skin_cannot_be_read_still_makes_a_shared_identity_give_no_head,
      an_add_dropped_by_a_full_channel_still_makes_a_shared_name_give_no_head,
      an_edited_line_frees_each_old_heap_string_once_with_its_size (M-4),
      a_typed_face_code_point_is_removed_from_a_line_without_a_head (edit_message writes a clean-only line).
Existing tests that expect a glyph now first do a successful flush (M-2 rule).

Results
-------
mise run test:  Summary 218 tests run: 218 passed, 0 skipped (after the follow-up)
mise run check: cargo check, clippy -D warnings, fmt --check all clean
mise run bench -- place_ (before -> after):
  place_chat_line              47.5 ns  -> 322.9 ns  [316.3 330.4]
  place_join_line_240_players  4.02 us  -> 3.88 us   (in noise)
  place_edit_chat_line         new         349.1 ns  (resolve + clean check + one String)

Concerns
--------
- place_chat_line is about 6.8x slower. M-1 causes this: by_xuid must now walk all 240 entries to prove that only one matches. Before, find() stopped at the match (index 100 in the bench). Past index 100 every bench XUID is 4 bytes long, so each compare is a memcmp. Real 16-digit XUIDs cost the same. This runs once per chat line, not per frame. I also measured a walk with no chain: 302 ns, about 8 percent better. It was reverted because the code was harder to read for a small gain. A XUID index would bring the cost back down if it matters.
- With the chat setting off, the follow-up removes face code points from chat lines. Text outside chat (signs, name tags) is not covered. This stays the known limit the review describes.
- When the mod is suspended, on_identity does nothing (the hooks are removed then anyway), so names seen while the mod is off are not noted.
- The exit flag is applied on the next page flush that gets the chat try_lock. Until then, the page stays as it was.

Task Completed
