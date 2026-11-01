# Ledger: plan docs/superpowers/plans/2026-10-31-chat-heads.md

Execution: subagent-driven. No per-task reviewers (operator rule). One whole-branch review in Task 11.
Briefs: scratchpad `briefs/task-N-brief.md` (Global Constraints + Review Focus + the task text).
Per-task results: `task-N.md` in this directory.

Pre-flight scan:
- Not a git repository: there are no commits or diffs. The final review uses the file list from the task ledgers.
- Tasks 4 to 7 all edit `heads/core/src/services/mod.rs` and the bench file, so they run in order.
- Task 1 writes docs only, so it runs in parallel with Task 3.
- Task 2 needs Task 1 RVAs and offsets. Tasks 8 and 9 need Task 1 and Task 2.

Task 3: complete (135/135, check clean; lead read the ledger and the new field).
Task 4: complete (140/140, check clean; has_face hit 816.5 ps, miss 1.404 ns. Ruling: the extra pub tuple field and FIRST/LAST_CODEPOINT consts are kept, because amml and later tasks build cells by index. Cost if wrong: a later rename).
Task 5: complete (145/145, check clean).
Task 6: fix round 1. Ruling: Localized names must start the line (after § codes and an optional '* '). Reason: an unanchored match gives a wrong face when a player name is a word of the line, the spec's top anti-goal. Cost if wrong: Task 1 may show a localized form with text before the name, which then gets no head.
Task 6: complete (159/159, check clean; place_chat_line 70.6 ns, place_join_line_240_players 4.31 us. Known: a player named like a word of the line, such as 'joined', makes those lines get no head. That is the safe result).
Task 7: complete (lead reran mise run test after the fmt pass: 164/164; asset 16384 bytes). Waiting on Task 1 before Tasks 2, 8 and 9.
Task 1: complete (findings in docs/findings/chat-heads-re/findings.md; the lead saved them, because the subagent could not write .md files).
Ruling: /me is a Chat line (type 1) with an empty username and the full string `* Name action`. Core treats a Chat line with an empty username like Localized (the name anchored after `* `). The XUID is not used for it. Reason: the plan's Chat rule puts the glyph before `*`, and the fallback has no name. Cost if wrong: /me gets no head.
Ruling: Decision 7 font check. Keep the BitmapFont check as planned (cheap, harmless). Task 10 adds a check with the Noto Sans chat typeface. If the faces then draw badly, do a follow-up for the typeface setting. Reason: no proof yet of which font object chat uses. Cost if wrong: one more round after Task 10.
Ruling: PlayerListAddEntry.mXUID +0x40 is an inference. Task 10 confirms it by behaviour: if chat lines get no head while join lines do, the offset is wrong. No per-entry log is added.
Ruling: the gui_message detour must not read argument 5 (filteredMessage) after the original returns. It reads only from `this`.
Task 6: fix round 2 complete (/me rule; 165/165, check clean).
Task 2: complete (derive 304/304 identical, lint 0 open, 166/166). CHAT has 7 entries (FontHandle.dtor added). Note: an amml clippy deprecation error in ui/kit/tests/widget/segmented.rs is outside this work and was not changed. Ruling: the char_width hook signature follows the findings (rcx, edx, r8b in; f32 out), not the signature tool.
Task 8: fix round 1. Ruling: the ChatState lives for the life of the process (a disable keeps it, an enable reuses it), so cells never move. The packet feed takes the blocking lock, so a face is not lost while the game thread uploads. Cost if wrong: none known.
Task 8: complete (180/180, check clean; install resumes the first state, suspend replaces uninstall, the packet feed uses with_state_blocking).
Task 9: 189/189, check clean. Ruling: remove the bench crate's dependency on amml (CLAUDE.md says benches depend on core); the width decision is CellFlags::has_face, which Task 4 benches. Accepted: the font check is cached and refreshed every 64 frames; HOOK_CAPACITY 8 is now full; a chat line that arrives while the packet feed holds the lock gets no head (rare, and it fails safe).
Task 9: complete (bench fix done; 189/189; core has_face hit 868 ps, miss 897 ps).
Task 10: in-game round 1. The heads show in the HUD and the chat window, and /me puts the head after '* '. Fault: the face was drawn as the full 16x16 cell (twice the text height), so it overlapped the lines above and below. Ruling: draw the face at its native 8x8 at cell x 0..7, y 4..11 (the vanilla hunger and armor icons fill y 3..11), with width 9.0. 189/189, check clean. Restarted the game as PID 36084.
Task 10: in-game round 2. 8x8 lined up but was smaller than the letters. Operator chose 10x10: nearest sampled at cell x 0..9, y 2..11, width 11.0. 189/189, check clean. The game runs as PID 38932 (DLL hash checked). Early-load screen: How to Play no longer exists in 26.60, so no E1 screen before a world is known. Faces drew in every fresh-start session.
Task 11: final review gave 0 critical, 2 important and 5 minor findings (final-review.md). Benches: has_face hit 828 ps, miss 849 ps; place_chat_line 56 ns; place_join_line_240_players 4.78 us.
Ruling: fix I-1 (strip U+E110..U+E1FF from line text), I-2 (record the identity of every Add entry in a bounded list, so a stale same-name player is ambiguous), M-1 (by_xuid unique), M-2 (no glyph before the first successful upload; mark the page dirty on world exit), M-3 (back off after failed uploads), and M-4 (heap free counters test) in one fix dispatch. The lead updates the spec for M-5. Known limit: a face code point typed into text outside chat (signs, nametags) still draws a face.
Task 11: fixes complete (I-1, I-2, M-1, M-2, M-3, M-4, plus cleaning while the setting is off). The lead reran: check clean, 218/218, package built. place_chat_line went from 47.5 ns to 322.9 ns because of the unique XUID check, once per line. This is accepted.
Installed into the main instance 1.26.60.05-standalone: the DLL, PDB and amml.hcl, plus recipes font.hcl, gui_message.hcl (new) and player_list_packet.hcl. Hashes checked. Backup: mods-backup/alaydriem-locator-heads-20261031-110122. The game was launched for the operator (PID 11276).
Task Completed
