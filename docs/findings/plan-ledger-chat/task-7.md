# Task 7: Glyph page (core)

## Files changed
- heads/core/src/services/glyph_page.rs (new)
- heads/core/src/services/mod.rs (module + re-export)
- heads/core/tests/services/glyph_page.rs (new, 5 tests)
- heads/core/tests/services/mod.rs (mod line)
- heads/amml/assets/glyph_e1_row0.rgba (new, 16384 bytes)

## Asset method
Script: scratchpad/task7/extract.py (Python 3 -I, Pillow 12.3.0). Read
`%LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads\data\resource_packs\vanilla\__brarchive\font.brarchive`
read-only. Header count at offset 8; entries 256 bytes from offset 16; data offset/size u32 LE at
entry+248/+252; data base = 16 + count*256. Found `glyph_E1.png`, decoded to RGBA, asserted 256x256,
wrote the first 16 pixel rows (16384 bytes). 1578 bytes of row 0 have non-zero alpha. Nothing was written
into the game instance.

## Tests
`mise run test`: 164 tests run: 164 passed, 0 skipped (before a rustfmt-only change to the test file).

## Check
`mise run check`: cargo check, clippy -D warnings and fmt --check all clean (after `cargo fmt -p locator-heads-core`).

## Concerns
- None. Extra public const `GlyphPage::CELL_SIDE = 16` added beyond the brief.
- Failing-first run was skipped; tests and impl were written together.

Task Completed
