# Task 6: Chat line placement (core)

## Files changed

- Created `heads/core/src/services/chat_head_placer.rs`
- Created `heads/core/src/structs/chat/message_kind.rs`, `heads/core/src/structs/chat/chat_line.rs`
- Created `heads/core/tests/services/chat_head_placer.rs` (11 tests)
- Edited `heads/core/src/structs/chat/mod.rs`, `heads/core/src/structs/mod.rs`, `heads/core/src/services/mod.rs`, `heads/core/tests/services/mod.rs`
- Edited `heads/bench/benches/heads.rs` (benches `place_chat_line`, `place_join_line_240_players`)

## Results

- `mise run test`: `Summary [0.609s] 156 tests run: 156 passed, 0 skipped`
- `mise run check`: clean (cargo check, clippy -D warnings, fmt --check)
- `place_chat_line`: 70.6 ns (95% CI 69.4 to 71.8 ns)
- `place_join_line_240_players`: 5.51 us (95% CI 5.40 to 5.63 us)

## Notes

- A `Â§X` pair directly before a name counts as a word boundary. The offset is the name start.
- A Localized name counts only if it occurs once in `full`. A second occurrence of any other player, or a duplicate directory name, gives `None`.
- `place` and `glyph_for` do not allocate. `insert` allocates once.

## Concerns

- The join-line cost is linear in directory size (5.5 us at 240 players, once per Localized line, not per frame).

## Fix round: Localized anchor

- A Localized name must start at the line anchor: any number of leading `§X` pairs, then an optional `* `. A name elsewhere in the line gives `None`.
- Two directory names that both match at the anchor give `None`. A directory name that is a word elsewhere in the line also gives `None`.
- New tests: `a_player_named_like_a_word_in_the_line_gets_no_head`, `a_name_in_the_emote_body_gets_no_head`, `two_names_that_both_fit_the_start_of_the_line_give_no_head`.
- `mise run test`: `Summary [0.881s] 159 tests run: 159 passed, 0 skipped`. `mise run check`: clean.
- `place_join_line_240_players`: 4.31 us (95% CI 4.18 to 4.46 us).
- Concern: a player named like a common word (`joined`) makes every join line that contains that word give no head. This is the safe result.

## Fix round 2: emote as a Chat line

- A Chat line with an empty username uses the anchored-name rule of Localized. The XUID is not used for it. Chat lines with a username and Whisper lines are unchanged.
- New test: `an_emote_arrives_as_a_chat_line_without_a_username`.
- `mise run test`: `Summary [0.669s] 165 tests run: 165 passed, 0 skipped`. `mise run check`: clean.

Task Completed
