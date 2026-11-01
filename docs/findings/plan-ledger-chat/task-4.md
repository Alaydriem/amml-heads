# Task 4: Glyph cells (core)

## Files changed
- Created `heads/core/src/structs/chat/glyph_cell.rs`, `heads/core/src/structs/chat/mod.rs`
- Created `heads/core/src/services/cell_flags.rs`, `heads/core/src/services/glyph_cells.rs`
- Created `heads/core/tests/services/glyph_cells.rs`
- Edited `heads/core/src/structs/mod.rs`, `heads/core/src/services/mod.rs`, `heads/core/tests/services/mod.rs`
- Edited `heads/bench/benches/heads.rs` (benches `has_face_hit`, `has_face_miss_ascii`)

## Results
- `mise run test`: 140 tests run: 140 passed, 0 skipped (5 new glyph_cells tests)
- `mise run check`: clean (check, clippy -D warnings, fmt)
- `has_face_hit`: 816.5 ps (median, [806.1 ps 827.4 ps])
- `has_face_miss_ascii`: 1.404 ns (median, [1.366 ns 1.445 ns])

## Concerns
- `GlyphCell` has a public tuple field and `FIRST_CODEPOINT`/`LAST_CODEPOINT` consts (extra to the brief).
- An extra test, `full_is_not_reported_before_a_refusal`, was added.

Task Completed
