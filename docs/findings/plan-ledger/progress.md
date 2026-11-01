# SDD ledger — plan: docs/superpowers/plans/2026-10-30-locator-heads-26.60.md

Spec: docs/superpowers/specs/2026-10-30-locator-heads-design.md
Operator overrides (2026-10-30): no per-task reviewer, one final review only; tests run in a private instance `1.26.60.05-locator-heads`; track and stop only our own PID; no commits.
Workspace: locator-heads is not a git repo, so there is no worktree. The amml repo was at 8f0e881 at the start.

## Preflight scan

| Rows | Produces / consumes | Finding |
| --- | --- | --- |
| T1 -> T2, T4, T7, T8 | R1-R12 table | T4 `pack` needs R2; T7 offsets need R8-R10; T8 needs R1, R3-R6, R11. Consistent. |
| T2 -> T7, T8, T9 | entry ids | T9 `amml.hcl` lists the T2 ids plus existing ones. Consistent. |
| T3 -> T5, T7 | FaceImage, SkinView, FaceExtractor | Consistent. |
| T4 -> T8 | TagCodec, QuadScaler, IconRect | Consistent. |
| T5 -> T8, T9 | FaceStore<T>, FaceUpdate, HeadsSettings | Consistent. |
| T8 -> T9 | RenderState, detours | Consistent. |
| T2 self | modifies shared amml recipe files | Other agents work in amml. See ruling 1. |
| T9 self | test `installs_six_hooks_on_2660` | Counts 6 hooks, which agrees with its own text. |
| T10 self | uses private instance | Agrees with Global Constraints after the operator edit. |

Ruling: In Task 2, new entries for existing classes are appended at the end of the existing amml recipe file, and no other line is touched. New classes get new files — this follows the amml rule "one HCL file per game class" and limits conflicts with other agents — if wrong, a merge conflict in amml/recipes that the operator resolves by hand.
Ruling: Subagents cannot write report files (the harness refuses). Implementers return the report text, and the lead saves it as `task-N-report.md` — no information is lost; the lead's context grows a little.

## Tasks
Task 3: complete (8/8 tests, check clean). Minor (deferred): CLAUDE.md copy still has zoom-specific sections; red step skipped.
Task 5: complete (17/17 tests on lead re-run, check clean). Note: a failed upload is not retried until the next Skin/Add update (the vanilla dot stays). CLAUDE.md is rewritten for locator-heads.
Task 1: complete (findings: docs/findings/re-confirm/findings.md). R1, R4, R8, R10 proven; R12 inference only, checked in Task 10.
Ruling: Drop the `ResourceLocation.dtor` entry. Free mPath with amml-msvc MsvcString on the game heap (R3: the dtor is MCFOLD and inlined) — if wrong, a leak of one string per upload (bounded by the texture-name ruling).
Ruling: The packet handler hooks resolve through the existing `LegacyClientNetworkHandler.vtable` entry, slots 179 and 107 (R8: the ClientNetworkHandler vtable holds empty stubs there) — if wrong, the hooks never fire and the in-game log shows 0 faces.
Ruling: Texture names are per store slot (`locator_heads/slot_<0..127>`), not per actor and generation, because unloadTexture leaves the map node behind (R5). This keeps the game's texture map bounded at 128 extra nodes — if wrong (re-upload to a name already used does not replace it), Task 10 shows stale faces, and the fix is updateTextureInPlace (0x5421340).
Ruling: The face texture colour is 0xFFFFFFFF, which takes the callee's white path (R1), the same path a vanilla white waypoint takes — no extra cost.
Task 4: complete (25/25 tests on lead re-run). pack samples: (1,1,1,1) -> 0xFFFFFFFF; (0.5,0.25,0.999,1) -> 0xFF7F3FFE; (0,0,0,0) -> 0x00000000; clamp (-0.5,2,0.5,1) -> 0xFF00FF7F. Extra test translucent_tag_colour_is_not_a_tag.
Task 6: complete. Medians: extract_64 5.32 us, extract_256 5.74 us, decode_tag 3.40 ns, texture_for_hit_128 12.17 ns (limit 100 ns). Minor (deferred): criterion 0.5 with default features off.
Ruling: The moved-offset field entry ids are `PlayerListAddEntry.size`, `PlayerListPacket.variantIndex`, `PlayerListAddEntry.mSkin`, `SerializedSkinImpl.mSkinImage` — fixed now so Tasks 2 and 7 agree while they run in parallel — if wrong, a rename in one file.
Task 7: complete (46/46 tests on lead re-run). Readers are `unsafe fn` (they read raw addresses); added MemoryReader. Inferred: ListNodeLayout next@0 prev@8 (standard MSVC list node). Minor (deferred): the drop log fires on drop 1, then every 256.
Ruling: Add the global entry `ClientEntityWaypoint.vtable` (`0xF13DFD0`) to Task 2, because the tagger needs it to tell player waypoints from plain ones and the plan had no entry for it — if wrong, use a virtual call to tryGetActorID (slot 7) instead.
Ruling: TextureBridge owns a pool of 128 texture names (`locator_heads/slot_<i>`) with a free list; a GameTexture holds its name index and returns it to the pool on unload — this follows the slot-name ruling without changing the FaceStore API — if wrong, a pool leak shows as uploads that fail after 128 faces.
Task 8: complete (73/73 tests on lead re-run). The lock is held per step, never across a game call (confirmed). after_render() uses the group fetched in before_render. Risks for Task 10: getTextureGroup hidden-return convention; getTexture args (0u8, 0u64, 0u32).
Ruling: A NaN waypoint colour packs to 0 lanes in the mod but to 0x80000000 in the game — left as is, because a NaN colour from a server is pathological and only changes that icon's tint — if wrong, one wrongly tinted dot.
Task 9: complete (92/92 tests on lead re-run; package has 3 files; only the 4 amml_mod_* exports). Waits on Task 2 for the ClientEntityWaypoint.vtable recipe.
Ruling: disable() leaves up to 128 face textures loaded until the game exits, because no client or texture group is reachable at disable — accepted; disable normally runs only at shutdown — if wrong, at most 512 KiB of GPU textures leak after a manual disable.
Task 2: 16 of 17 entries done (all resolve on 26.60 and 26.51; 0 moved; lint 0 open; signature for drawIcon matches R1). `amml mise run check` fails at fmt in another agent's uncommitted file (derive/cli recipe_lint.rs), which is not ours. ClientEntityWaypoint.vtable was asked for again.
Ruling: amml `mise run check` failing on another agent's uncommitted fmt change is not a Task 2 failure, because our change is HCL only and `mise run test` passes (1543) — if wrong, the operator sees a red check that is not ours.
Ruling: The private instance gets a snapshot copy of amml/recipes at launch time; live recipe edits by other agents (one entry, MinecraftUIRenderContext.flushText, fails right now) affect only the mods that list them — if wrong, the loader reports that entry missing for another mod, which our instance does not load.
Task 10 prep: done. The trace logs are in (92/92 tests). Script amml/scripts/ingame-standalone-locator-heads.sh plus a mise task appended (10 lines). Private instance %LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads created (no mods, no cache). The PID file is locator-heads.pid. The game is not started yet; it waits for the ClientEntityWaypoint.vtable recipe fix.
Task 2: complete (17 entries; derive 254/254 on both builds, 0 moved, deterministic; lint 0 open). ClientEntityWaypoint.vtable has one recipe plus a section check, because no second recipe that obeys the rules exists (only the dtor refers to it, and every pattern there is ambiguous).
Ruling: One recipe plus a check is accepted for ClientEntityWaypoint.vtable — recipes.md allows a checked single route, and the alternative breaks the rule that patterns end at a call — if wrong, on a future build the entry refuses (it cannot be wrong silently, because the pattern holds the [rax+78h] actor-id store).
Task 10 run 1 (script): load checks pass (resolved 38/38, missing 0, features off: none, both mods loaded, manager linked to locator-heads). The script stopped the game after its fixed wait; no crash. The trace lines did not show (log level above trace), so R12 is still open.
Task 10 run 2 (lead, at the operator's request): game relaunched in the private instance, PID 46460 (created 2026-10-30T22:49:06.0872060Z) recorded in locator-heads.pid. Running for the operator's manual checks.
Finding (shared tooling): amml/scripts/ingame-mods.sh `check_fresh` hangs when a dep-info file is over 64 KB (the manager's is 65,593 chars), because it reads through a here-string. Our script has a file-based copy; the shared script is not edited. Other ingame-* tasks will hang until it is fixed.
Task 10 Step 5: locator-heads test 92/92; modmanager-amml test 1039/1039; modmanager-amml e2e 9/9; amml test 1597 passed, 9 skipped. No failures.
Task 10: waiting on the operator's manual visual checks (a)-(g) and on R12.
Final review: Ready with fixes (docs/findings/final-review/findings.md). One fix wave: I1, I3, M1, M2, M4, M7, M8, plus the detour thread-id lines at info level for I2.
Ruling: The one-time detour thread-id lines move from trace to info level (6 lines per session), because I2 needs them and the AMML logger has no per-mod trace switch at hand — if wrong, 6 extra info lines per session.
Ruling: M3 (raw tag colour after a panic, for one frame) and M5 (format! in pre-link logs) are accepted as is — cosmetic, and the logger has no fields before the link — if wrong, one oddly tinted frame or two unstructured log lines.
Final fix wave: complete (99/99 tests on lead re-run; check clean; package rebuilt). I1, I3, M1, M2, M4, M7 fixed; M8 was already correct; detour thread-id lines now at info level. The script guard has two layers (the pid file plus any game in the instance); lead-inspected, syntax-checked only.
Ruling: No scoped re-review of the fix wave — the operator rule limits review to one; the lead checked the tests and the I3 guard by reading it — if wrong, a defect in the fix wave is found only in game.
Open: I2 / R12 (thread ids) needs one launch of the new build; the operator's manual checks (a)-(g) are pending.

## Debug: creator skins (2026-10-30)
Report (operator): faces work for classic skins but not for creator (Marketplace) skins.
Hypotheses (not yet tested): H1 the skin is rejected by FaceExtractor (format, size or a short blob), so the vanilla dot stays; H2 custom geometry puts the head UV somewhere else, so the wrong part of the skin is shown; H3 the face is in an animated face layer (mSkinAnimatedImages) or the base face region is blank; H4 the skin arrives later or differently (PlayerSkinPacket, trusted-skin gate) than for classic skins.
Step 1: instrumentation (a per-skin info log plus a PNG dump behind heads.debug_dump) is being added.
Step 1 done: SkinDiagnostics (105/105 tests). Offsets computed from the header, not proven: mResourcePatch 0x40, mDefaultGeometryName 0x60, mSkinAnimatedImages 0xE0 (element 0x40), mIsPremium 0x1E9, mIsPersona 0x1EA.
Diagnostic build installed in the private instance with heads.debug_dump = true; game PID 55428 (created 2026-10-31T01:39:44.4310640Z) recorded in locator-heads.pid. Load is clean (missing 0, features off: none). Waiting for the operator to join with creator-skin players.
Evidence (run 2026-10-31 01:39, PID 55428): the "creator" players are persona skins: 256x256, format 4, size matches, is_persona true, geometry.persona_<hash>, one animated image of type 1 (Face). Extraction says "Ok", but the classic UV (32,32 at s=4) is empty atlas space, so the face is transparent and draws as nothing (no dot, no face). Classic 64x64 skins work. R12 resolved: render, draw_icon, player_list, client_update and exit_level all run on ThreadId(1).
Root cause: H2 confirmed. Persona skin atlases place the head by geometry, not at the classic UV. H1 and H4 rejected (Ok outcome, PlayerList.Add path); H3 is not the cause (the base atlas has the pieces; the animated layer is only the blink).
Ruling: Persona skins are now in scope (the spec listed them as a non-goal; the operator reports this as a gap). Approach: serialize mGeometryData with the game's Json::Value::toStyledString (one function entry), parse in core with serde_json, take the head bone's north face (box UV or per-face UV) and the hat bone, scaled by image size / texture_width; fall back to the classic UV — reading the game's JSON through its own writer avoids mirroring the jsoncpp map layout — if wrong (the writer is inlined), walk the Json::Value tree in memory instead.
Persona fix, core part: complete (122/122 on lead re-run). GeometryHeadLocator (1.12 and 1.8 formats, box UV and per-face north UV), FaceExtractor::extract_at, FaceError::EmptyFace (so a transparent face shows the vanilla dot). Calls by the implementer, accepted: the 1.8 format falls back to the first geometry; an out-of-bounds hat is dropped; extract_at accepts any width (SkinReader still caps each side at 4096).
Persona fix, game part: entries JsonValue.toStyledString (26.60 0xe932900; rcx = this, rdx = hidden out std::string; game-heap string) and SerializedSkinImpl.mGeometryData (0xF8; 0x118 on 26.51). derive 256/256 on both builds, 0 moved; lint 0 open.
Persona fix, wiring: complete (130/130 tests; check clean; package). The red step was skipped this round (implementer note). Caveat: if extract_at fails for a skin that has geometry, the classic cut is not tried; the vanilla dot stays.
Persona build installed; recipes synced into the private instance; game PID 61224 (created 2026-10-31T01:51:54.9584500Z) in locator-heads.pid. Load: resolved 40/40, locator-heads 26/26, features off: none. debug_dump still on; the data dir was cleared before the run.
