Task 9 report: chat detours, font check, optional chat hook group

Files created
- heads/amml/src/detours/gui_message.rs (amml_detour!: original with all 10 args unchanged, then ChatRuntime::edit_message(this); reads only `this`)
- heads/amml/src/detours/char_width.rs (amml_detour!: font rcx, character i32, force_unicode u8 -> f32; GlyphCell::WIDTH when ChatRuntime::has_face, else original)
- heads/amml/src/layouts/gui_message_layout.rs (GuiMessageLayout, 0x110; line(), filtered_line(), replace_full(), replace_filtered_full(); new string built before the old block is freed)
- heads/amml/src/services/chat_font_check.rs (ChatFontCheck::new(slot, dtor, bitmap_vtable), unsafe is_bitmap(client); HANDLE_SIZE 0x50, DEFAULT_FONT 0x30; FontHandle destroyed with FontHandle.dtor every call)
- tests: tests/detours/gui_message.rs, tests/detours/char_width.rs, tests/layouts/gui_message_layout.rs, tests/services/chat_font_check.rs
- test support: tests/support/fake_heap.rs, fake_gui_message.rs, fake_font.rs

Files changed
- heads/core/src/structs/chat/glyph_cell.rs: GlyphCell::WIDTH = 16.0
- heads/amml/src/services/chat_runtime.rs: set_game_access(font_check, heap), unsafe refresh_font(client), unsafe edit_message(message), FONT_CHECK_FRAMES = 64
- heads/amml/src/detours/client_update.rs: calls ChatRuntime::refresh_font(this) after flush_page
- heads/amml/src/detours/mod.rs, layouts/mod.rs, services/mod.rs: new modules
- heads/amml/src/heads_mod.rs: chat_hooks() -> [HookSpec; 2]; chat_parts() resolves the 7 CHAT entries and the 2 hook targets; enable installs the 6 locator bar hooks, then the chat group; on any chat failure warns "features off: chat heads (<reason>)" and re-sets the observer without the chat offsets; on success ChatRuntime::set_game_access + ChatRuntime::install, info "features off: none". disable calls ChatRuntime::suspend() (state never dropped)
- heads/amml/src/services/hook_installer.rs: install removes only the hooks that call put in (remove_since); remove_all unchanged in effect
- heads/amml/src/services/heads_runtime.rs: hook_count(), take_hooks_since(count); take_hooks = take_hooks_since(0)
- heads/amml/src/types/hook_spec.rs: dropped "Every hook is required" from the comment
- heads/amml/tests/heads_mod.rs: six-hook test now expects 6 + 2 chat hooks; new a_missing_chat_entry_turns_off_only_chat_heads (loops over all 7 CHAT entries), a_chat_hook_that_fails_comes_out_and_the_locator_bar_stays; unhook order now 8..1
- heads/amml/tests/support/fake_host.rs: chat entries in for_2660, with_failing_hook(entry)

MANIFEST vs CHAT: no conflict. The CHAT entries stay in MANIFEST/amml.hcl (AMML docs: the mod decides in code which entries are required). refuses_when_an_entry_is_missing still passes: a locator bar entry fails enable before chat is looked at. A missing CHAT entry never fails enable.

Font check cache: ChatRuntime::refresh_font runs from the ClientInstance::update detour. It calls getFontHandle on the first frame after set_game_access and then once every 64 frames (about 1 s at 60 fps); other frames cost one relaxed atomic add. The answer is a relaxed AtomicBool that edit_message reads; no getFontHandle call per chat line. A font change can take up to 64 frames to show.

Results
- mise run check: clean (cargo check, clippy -D warnings, fmt --check)
- mise run test: Summary [1.287s] 189 tests run: 189 passed, 0 skipped (after the bench fix)
- Red step before implementation: integration target failed to compile (7 errors: missing char_width, gui_message, GuiMessageLayout, ChatFontCheck, set_game_access, refresh_font)

Bench (lead ruling: the bench crate depends on core only; the char_width decision is CellFlags::has_face, benched by the Task 4 core benches; mise run bench has_face)
- has_face_hit        time: [855.11 ps 868.02 ps 882.68 ps]
- has_face_miss_ascii time: [883.47 ps 896.78 ps 911.24 ps]
An earlier run through ChatRuntime::has_face (OnceLock get + the same check) gave 1.61 ns hit and 1.57 ns miss; those benches and the bench dependency on locator-heads-amml were removed.
No allocation, no lock on the width path.

Concerns
- Font check proves only the getFontHandle default font, not the font chat draws with (findings Concern 3). The smooth font check is for the in-game test.
- refresh_font uses the client_update `this` as the ClientInstance, the same assumption Task 8 made for the texture group.
- FONT_IS_BITMAP stays as last measured after disable; harmless because the hooks are removed and the state is suspended.
- A packet that arrives between the chat hooks going in and ChatRuntime::install (first enable only) feeds no state; no world is loaded at enable, so expected to be empty.
- HOOK_CAPACITY is 8 and the mod now uses all 8; a further hook needs the capacity raised.
- edit_message uses with_state (try_lock): a line that arrives while the packet feed holds the lock gets no head.
- The ChatRuntime::has_face wrapper (OnceLock get) has no bench of its own in the repo; only the core CellFlags::has_face is benched.

Task Completed
