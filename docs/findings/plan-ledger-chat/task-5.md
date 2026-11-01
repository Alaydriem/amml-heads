# Task 5: Player directory (core)

## Files changed
- heads/core/src/services/player_directory.rs (new)
- heads/core/src/services/mod.rs (module and re-export)
- heads/core/tests/services/player_directory.rs (new, 5 tests)
- heads/core/tests/services/mod.rs (mod line)

## Test result
`mise run test`: 145 tests run: 145 passed, 0 skipped

## Check result
`mise run check`: cargo check, clippy -D warnings and fmt --check all clean.

## Concerns
None. Entries are stored as a tuple (uuid, xuid, name) to keep one struct per file.

Task Completed
