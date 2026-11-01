# Task 3: heads.chat setting

Files changed:
- heads/core/src/config/heads_settings.rs (chat field, new(enabled, scale, chat), default_chat)
- heads/amml/descriptor.hcl (field heads.chat)
- heads/core/tests/config/heads_settings.rs (chat default, missing-key read, new callers)
- heads/amml/tests/services/descriptor.rs (field list)
- heads/amml/tests/detours/draw_icon.rs, heads/amml/tests/services/render_state.rs (new callers)

Tests: 135 tests run: 135 passed, 0 skipped (mise run test)
Check: mise run check clean (after cargo fmt on draw_icon.rs)
Concerns: none

Task Completed
