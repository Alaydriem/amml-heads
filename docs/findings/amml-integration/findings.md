# locator-heads: AMML and AMM integration findings

Read-only research, 2026-10-30. Target: Bedrock 26.60.05, Windows client, standalone AMML host.
Paths: `amml/` = `C:\Users\charl\projects\levi\amml`; other repos are siblings under `C:\Users\charl\projects\levi\`.
Only read-only commands ran: `amml-derive.exe inspect ...` (existing release binary) and read-only scans of the 26.60.05 exe. No game was started and no `mise run ingame-*` task was run.

## 0. Short answer
- An AMM client mod is a Rust `cdylib` with `amml_mod!`, `descriptor.hcl`, `amml.hcl` and a `ManagerLink`. `zoom` is the smallest real example (816 lines in the mod crate).
- Hook targets are entry ids from `amml/recipes/*.hcl`. Derived values are stored per BuildId.
- **26.60.05 BuildId is `6ad79ee4-17d51891a1119914`** (`inspect build-id`; `amml/docs/findings/26.60-cold-derive/results.md:3`). Data file: `amml/data/6ad79ee4-17d51891a1119914.hcl`. `6aa482fd-2658118eba0dbac5` is 26.51.
- **`HudLocatorBarRenderer` has no RTTI name in the 26.60.05 exe.** About 1770 RTTI-like names; none contains `Hud`, `Locator` or `ClientInstance`. `inspect vtable --class '.?AVHudLocatorBarRenderer@@'` prints "class ... not found".
- The manager overlay (egui) can upload RGBA and draw a face every frame. It draws on top of the finished game frame, in back-buffer pixels. It does not know the game GUI scale.
- No mod or SDK wrapper calls `MinecraftUIRenderContext`. Only plan-peek designs it (`amml/docs/superpowers/plans/2026-10-30-amml-plan-peek.md:83-146`); the `peek` folder is empty.
- Entries exist for `NetworkPeer.receivePacket` and `ClientNetworkHandler.handle.*`. None for PlayerListPacket, PlayerSkinPacket or LocatorBarPacket.

## 1. Structure of an AMML mod that is also an AMM client (example: zoom)

**Cargo**
- Workspace members `zoom/core`, `zoom/amml`, `zoom/bench` (`zoom/Cargo.toml:1-3`). Edition 2024, rust 1.98.
- Path deps (`zoom/Cargo.toml:15-18`): `amm-client` (default-features = false: no egui), `amm-core` (both `../modmanager-amml/crates/`), `amml-sdk` (`../amml/sdk/sdk`).
- `zoom/zoom/amml/Cargo.toml:11-13`: `[lib] name = "alaydriem_zoom"`, `crate-type = ["cdylib","rlib"]`, `autotests = false` with one `integration` target, `[lints] workspace = true`. Workspace denies `unwrap_used` and `expect_used` (`zoom/Cargo.toml:30-32`).
- A mod that draws with egui keeps the default features of `amm-client` and depends on `egui = "=0.37.0"` (`minimap/Cargo.toml:22-37`).
- Logic in `zoom-core` (no unsafe); `zoom-amml` has `#![deny(unsafe_code)]` (`zoom/zoom/amml/src/lib.rs:4`) with local `#[allow(unsafe_code)]`.

**`amml_mod!`**
- `zoom/zoom/amml/src/lib.rs:14`: `amml_sdk::amml_mod!(ZoomMod);`. Macro at `amml/sdk/sdk/src/macros.rs:8-46`: exports `amml_mod_load`, `_enable`, `_disable`, `_unload`, each under `catch_unwind` in `ModGlue`.
- `AmmlMod` trait: `amml/sdk/sdk/src/amml_mod.rs:106-130`. `NAME` must equal `name` in `amml.hcl`. zoom: `zoom_mod.rs:83-101`.
- Only `load` and `enable` may install hooks (`api.rs:78-79`). On the standalone host these run before game static initializers: no game calls, no `Api::heap_new` (`amml_mod.rs:113-123`).
- `ModDirs::new` gives `mod_dir`, `config_dir`, `data_dir` (`amml/sdk/sdk/src/mod_dirs.rs:3-21`).

**`amml_detour!` and `DetourGuard`**
- `amml_detour!(pub mod name(args)->Ret |original| {body})` (`macros.rs:64-99`): module with `GUARD: DetourGuard` and `extern "C" fn detour`. Return type must be `Copy`. Pointers as `usize`, bools as `u8`.
- zoom detours: `zoom/zoom/amml/src/detours/mod.rs:13-65`. `client_update(this, init_finished: u8, extra: u64) -> u8` (`:13-19`); 26.60 added a bool in r8, passed through as raw `u64`.
- `DetourGuard` (`amml/sdk/sdk/src/detour_guard.rs:145-199`): holds the `original` slot; `run()` catches a panic, logs once, disables the whole mod and runs the original.
- Hand-written detours with `GUARD.original_fn::<F>()`: `lightlevels/levels/amml/src/detours/extract_name_tags.rs:7-27`, `stratum/stratum/amml/src/detours/container_open.rs:9-29`.
- Install: `Api::resolve(id)` (`api.rs:64`), then `unsafe Api::hook(target, detour, priority, &GUARD.original())` (`api.rs:85-111`). Higher priority runs first. `HookOrder::WRITE` = 0, `READ_AFTER_WRITERS` = 100 (`amml/sdk/game/src/structs/hook_order.rs`).
- zoom wrappers: `AmmlHost::hook` (`amml_host.rs:24-33`), `HookInstaller::install` (`hook_installer.rs:12-34`). Required entries abort; optional entries log and switch their feature off. Hook list: `ZoomMod::hooks()` (`zoom_mod.rs:24-69`).
- For a `vslot` target the mod reads the code pointer from a `global` vtable entry at run time: stratum `table_slot` (`stratum/stratum/amml/src/stratum_mod.rs:128-137`, used at `:147-198`).

**`amml.hcl` packaging**
- `zoom/zoom/amml/amml.hcl`: `name`, `entry` (DLL name), `api_version = 1`, `entries = [...]`. Parser: `amml/loader/runtime/src/services/mod_registry.rs:73-115`. `entries` lists every id the mod may use; code decides which are required (`lightlevels/levels/amml/amml.hcl` mixes both).
- `mise run build-mod` in zoom: `cargo build --release -p zoom-amml`, copies DLL and `amml.hcl` to `dist/alaydriem-zoom/`. minimap uses a `package` task (`minimap/mise.toml:41-53`).
- Loader finds a mod at `mods/amml/<folder>/amml.hcl`.
- **WARNING:** `zoom/mise.local.toml` sets `LM_MODS_DIR` to the play folder `...\levilauncher.exe\versions\1.26.51.01\mods`; its `install-mod` task writes there. Do not copy it.

**`descriptor.hcl` and registration**
- `zoom/zoom/amml/descriptor.hcl`: `id`, `display_name`, `version`, `author`, `config_path`; `claim "<Capability>" {mode, reason}`; `keybind "<id>" {label, default, capability, scope, config_path}`; `section` blocks with `field "<path>" {label, help, kind, default, apply}` (kinds: float, int, enum, bool, chord).
- Capabilities: `Camera, Input, Hud, Overlay, WorldDraw, RenderSize` (`amm-core/src/structs/descriptor/capability.rs:5-12`).
- Embedded with `include_str!` (`zoom/.../services/descriptor.rs:5`).
- freecam's `Hud` claim with `hide_vanilla_hud = true` hides the whole vanilla HUD (`amm-core/.../snapshot_builder.rs:50-61`).
- minimap asks for `texture_budget_mb = 512`, `texture_budget_count = 2200`.
- `WorldDraw` is only a claim (lightlevels, stratum); the draw is the mod's own hook.

**How `amm-client` resolves the manager table**
- `ApiTable::resolve()` (`modmanager-amml/crates/amm-client/src/link/api_table.rs:53-69`): `GetModuleHandleW("alaydriem-mod-manager.dll")`, `GetProcAddress("amm_get_api")`, call with `AMM_ABI_MAJOR` = 2 (`amm-abi/src/types/version.rs:2`). Null = `ManagerMissing`; other major = `AbiMismatch`. A short table leaves later slots null, so a newer call returns `Unsupported`.
- zoom links on the first `ClientInstance::update` after enable (`zoom_runtime.rs:60-75`). Copy this pattern.
- `ManagerLink::without_frame(table, descriptor, config_dir)` (`manager_link.rs:96-103`) for a mod that draws nothing; `ManagerLink::new(..., FrameHandler)` (`manager_link.rs:65-81`) for an egui pass (minimap `minimap/minimap/link/src/services/minimap_session.rs:40`, freecam `freecam/freecam/amml/src/services/freecam_session.rs:36`).
- Both install Curia logging forwarded to the manager (`<game>/logs/amm/<mod id>/`).
- Link calls: `config::<T>()` (`:241`), `events()` for `ModEvent` (`:236`: `Keybind{action,phase}`, `SettingsChanged`, `Key`), `acquire`/`release` (`:177`), `texture_budget`, `game_picture`, `set_render_size`, `request_screenshot`.
- Settings, config and keybinds are declared only in `descriptor.hcl`; the manager draws the panel and stores `config.hcl`.
- zoom's link: `ZoomLink` (`zoom_link.rs:21-80`). Manager `mise run build-mod` makes `dist/alaydriem-mod-manager/` (`modmanager-amml/mise.toml:42-56`).

## 2. Naming a game function to hook

**Entry ids and recipes**
- Ids like `Class.member` in `amml/recipes/<class>.hcl` (236 `entry` blocks). Shared Rust list: `amml/sdk/game/src/structs/entry_id.rs`. `EntryId::new` is public (stratum names its own).
- Kinds: `function`, `vslot`, `global`, `field` (`amml/docs/recipes.md:25-30`). `hooked = true` marks detoured function entries; `mise run signature` probes only those (`recipes.md:15-16`, `:233-246`). A vslot hook target is not marked `hooked`.

**Per-build data**
- `amml/data/<BuildId>.hcl`: `resolved "<id>" {kind,value,recipes}`, from `mise run derive`.
- Both data files are STALE: 120 `resolved` blocks (2026-10-29) against 236 entries; untracked. Re-run derive before comparing.
- At run time the loader derives in process from `<instance>/amml/recipes` and caches by recipe digest (`loader/runtime/src/services/recipe_digest.rs`, `resolver_chain.rs`). `make-standalone-instance` copies `recipes/`.

**Workflow to add a NEW entry** (`recipes.md`, `porting.md`, `skill/amml-repair/SKILL.md`)
1. Survey (`porting.md:62-73`): find the function, record how args and return travel, gather two independent pieces of evidence.
2. Find: `inspect vtable --rva <hex>` or `--class <RTTI> --from N --to M [--names]` (`--names` names only slots whose function loads a `virtual ...` string, `hunt-tools/progress.md:13`); `inspect xrefs --string "<text>"` and `--rva <hex>` (lists data pointers, finds vtable slots); `inspect callers`, `function`, `disasm`, `fields`, `explain <id> --recipes recipes`, `similar`; `inspect packets --create-packet <rva>` (id, name, vtable, getId, size, handler slot); `mise run decompile -- --exe <exe> --rva <hex> --out <file>` (Ghidra).
3. Pattern: `inspect draft <rva>` wildcards build-specific bytes; `inspect scan "<pattern>"` counts matches.
4. Recipe (`recipes.md:32-92`). Steps: `pattern`, `pattern_any`, `function_start_of`, `string_xref`, `rtti_vtable`, `ref`, `call`, `operand`, `displacement_to_slot`, `slot_of`, `slot_in`, `read_pointer`, `vtable_slot`, `add`, `member_displacement`, `alternative`. Two independent recipes plus a strong check (`recipes.md:135-143`). A vtable-only entry is refused (`recipes.md:111`). Wildcard every immediate/displacement/branch byte or `literal_ok`. Patterns end at a call. Checks: `function_start`, `leaf_start`, `in_vtable`, `in_vtable_of`, `section`, `range`, `entry_is`. `notes` on every entry. One name, one layout.
5. Verify on both builds: `mise run derive -- --exe <exe> --recipes recipes --out <tmp> --report <tmp> --compare <old data>` (no old value moves, two runs identical); `mise run lint -- --exe <exe> --recipes recipes --report <file>` (0 open); `mise run signature -- --exe <26.60 exe> --out sig.hcl [--compare sig-26.51.hcl]`; then `mise run check`, `test`, `game-test`.
6. Mirrors: hand-written `#[repr(C)]` with `offset_of!` asserts; entries only for what moves (`porting.md:5-35`).
7. Order: recipes are written against 26.51 first, 26.60 runs cold once (`porting.md:100-107`). For a 26.60-only mod, the 26.51 side may be left unresolved (decision for the plan).
8. Repair skill for hard entries: `amml/skill/amml-repair/SKILL.md`, Methods 0 to 6.

**RTTI facts for 26.60.05**
- `rtti_vtable`, `slot_in`, `in_vtable_of` exist; no recipe uses them; game classes mostly have no RTTI (`recipes.md:42`).
- Routes for classes without RTTI: constructor anchors (`lea rax,[rip+..]; mov [reg+..],rax` plus `operand rip_target`, `recipes.md:206-211`); string xref to a `virtual X::f` string then `slot_of`; `ref` an entry then `slot_of`.

**Leads for the locator bar (cross-check of the locator-bar-render agent's Ghidra dumps)**
- `inspect function 0x5745690` gives `0x5745690..0x574742c`; one data ref at `0xf2601f0`.
- `inspect function 0x5747be0` gives `0x5747be0..0x5748b62`; one data ref at `0xf260188`.
- On 26.51, `0x5745690` is inside a different function: both are 26.60 addresses.
- `inspect vtable --rva 0xf260100 --to 40` shows two 18-slot tables back to back: A at `0xf260100`, B at `0xf260190`. `0x5747be0` is slot 17 of A; `0x5745690` is slot 12 of B. Same class not proven; table boundaries are a guess. Verify with the constructors.
- Strings: `textures/ui/locator_bar_dot_0` to `_3`, `locator_bar`, `LocatorBarPacket`, `LocatorBarPacketPayload`, `LocatorBarWaypointPayload`. `xrefs --string "locator_bar"` finds one function `0x1c6ccc0`; `locator_bar_dot_0` finds 0 functions.
- No LocatorBar entry exists in `recipes/` or `sdk/game/`.

## 3. Drawing options

**(a) Manager overlay**
- Hooks swap-chain Present, draws with D3D11 after the game frame (`amm-overlay/src/present_hooks.rs`, `frame_flow.rs`). Overlay pixels land on top of the vanilla HUD.
- Mod holds a `Hud` or `Overlay` lease; host calls the frame callback once per layer per frame, on the render thread (`amm-abi/src/types/callbacks.rs:6-22`). Hud layers first, then Overlay (`snapshot_builder.rs:16-47`). Hud takes no input.
- Mod side: `FrameHandler::frame(&mut self, ctx: &egui::Context, input: &FrameInputView) -> FrameReply` (`amm-client/src/link/frame_handler.rs:8-26`). `textures_lost()` means upload again. Example: `minimap/minimap/link/src/services/minimap_frame_handler.rs:38-61`.
- Texture upload works: `ctx.load_texture(name, ColorImage::from_rgba_unmultiplied(size, &bytes), TextureOptions::NEAREST)`, `handle.set(...)` to change (`minimap/minimap/ui/src/map/layer_textures.rs:45-52`). Bridge: `amm-core/src/services/egui_bridge/mesh_flattener.rs:73-96`, `amm-abi/src/types/texture_delta.rs`. Draw with `ui.painter().image(...)` in egui points.
- Limits (`amm-core/src/config/budgets.rs:40-68`): 64 MiB texture bytes per mod, 1024 textures, 16 MiB upload per frame, 256 deltas per frame, 4096 freed ids per frame, 262144 vertices, 786432 indices, 16384 draw commands. Descriptor may ask more (`texture_budget_mb`, `texture_budget_count`; caps 1 GiB / 4096, `texture_granter.rs:33-54`). A 64x64 RGBA face is 16 KiB.
- Coordinates: back-buffer pixels (`frame_flow.rs:410`). `AmmFrameInput` has `width`, `height`, `pixels_per_point` (window DPI / 96, not GUI scale; `frame_flow.rs:68, 422`), `time_secs`. No GuiData / GUI scale entry exists (plan-peek risk, `plan-peek.md:48`). To line up with the bar, the mod must get the dot position from a hook on the game render function.
- `hide_vanilla_hud = true` hides the whole HUD (`amm-amml/src/services/hud_control.rs:20-49`); does not fit. Suppressing the dot inside a game-side detour is the finer option (untested).
- Threads: frame callback on the render thread; game-thread hooks elsewhere. Hand over with a non-blocking lock or atomics. `game_picture` and `set_render_size` return `Busy` inside a frame callback.

**(b) Game-side drawing wrapped today**
- `Mesh.renderMesh`, `Mesh.*`, `MeshData.*` (stratum ghost blocks in `LevelRenderer.renderLevel`).
- `Tessellator.*`, `BlockTessellator.*`, `ScreenContext.tessellator`.
- `NameTagRenderObject.size`, `BaseActorRenderer.*`, `LevelRendererPlayer.extractNameTags` (lightlevels).
- `Font.getLineLength` (vslot), `FontHandle.*`, `ClientInstance.getFontHandle`.
- `HudScreenController.vtable/tick/handleSlotSelection`, `ScreenController.dtor`.
- `ActorRenderDispatcher.render`, `LevelRendererPlayer.queueRenderEntities/setupCamera/createViewRenderObject/getFov`.
- Not wrapped: `MinecraftUIRenderContext` (`fillRectangle`, `drawRectangle`, `drawImage`, `flushImages`, `drawText`), `ItemRenderer::renderGuiItemNew`, any dynamic texture in UI.
- stratum recon (`stratum/docs/findings/stratum-preview-recon/in-game.md` section 1): a mod cannot make its own render target or viewport; names `MinecraftUICustomRenderer::render(MinecraftUIRenderContext&, ...)` as the UI custom renderer entry.

## 4. Players, skins and packets
- Existing: `recipes/player.hcl`, `local_player.hcl`, `actor.hcl`, `level.hcl`; `Actor.stateVectorComponent`, `getAttachPos`, `mLevel`, `ClientInstance.getLocalPlayer`. No entry for the player list, names, UUID/XUID or skin.
- Packets: `MinecraftPackets.createPacket`, `Packet.read` (slot 8), `ClientNetworkHandler.vtable`, `LegacyClientNetworkHandler.vtable` (the live handler); handle slots for CameraInstruction, CameraPresets, NetworkChunkPublisherUpdate, PrimitiveShapes, ItemStackResponse, ContainerOpen, ContainerClose, MovePlayer, NetworkStackLatency (`recipes/client_network_handler.hcl`). Raw: `NetworkSystem.*`, `NetworkConnection.mPeer`, `NetworkPeer.receivePacket`. No PlayerList / PlayerSkin / AddPlayer / LocatorBar mirror.
- Worked packet-hook example: stratum `container_open` (`stratum_mod.rs:147-198`, `detours/container_open.rs:9-29`).
- DejaVu groundwork (plan-9a, `amml/docs/superpowers/plans/2026-10-30-amml-plan-9a-dejavu-groundwork.md:95-175`): entries and wrappers only, no working observer.

Packet table (`amml/docs/findings/hunt-tools/packets-2660.txt`, `packets-2651.txt`):

| Packet | id | 26.60.05 vtable / size / handler slot | 26.51 slot |
| --- | --- | --- | --- |
| PlayerListPacket | 63 | `0xf115810` / `0x50` / 179 | 171 |
| PlayerSkinPacket | 93 | `0xf134a70` / `0x98` / 107 | 100 |
| AddPlayerPacket | 12 | `0xf1b6770` / `0x1f8` / 237 | not checked |
| LocatorBarPacket | 341 | `0xf158e30` / `0x50` / 38 | 32 |
| UpdatePlayerGameTypePacket | 151 | `0xf1c0780` / `0x50` / 231 | not checked |

- None is an entry. LocatorBarPacket slot 38 is outside the 120-180 window of existing `handle.*` entries; verify before use.
- To observe: add vslot entries like `ClientNetworkHandler.handle.PlayerListPacket` (copy `ContainerOpenPacket`, `client_network_handler.hcl:173-195`); add `#[repr(C)]` mirrors (`inspect fields` on ctor/dtor); install in `enable`; copy out only what is needed, bounded.
- Alternative: hook `NetworkPeer.receivePacket` and parse raw bytes for ids 63 and 93 (not proven in game).

## 5. Test instances and in-game tasks
- `%LOCALAPPDATA%\amml\instances\`: `1.26.51.01-ll`, `1.26.51.01-standalone`, `1.26.60.05` (pristine copy, empty `mods`), `1.26.60.05-standalone` (patched exe, `amml.dll`, `amml/recipes`, `amml/cache`, `amml/logs/`; `mods/amml/` holds freecam, levels, minimap, mod-manager, stratum, zoom, amml-testmod).
- Rules (`amml/CLAUDE.md:92-101`): test only from these instances; never write to `%APPDATA%\levilauncher.exe\versions\`; end only the PID you started (`scripts/find-game-pid.ps1`); never `taskkill /IM`.
- `make-standalone-instance --game --instance` (`amml/mise.toml:196-233`): builds loader and patcher, copies the game without LeviLamina, patches from `.bak`, copies `amml.dll`, `recipes/`, `mods/amml`.
- `ingame-standalone-freecam` (`:1025-1034`, `scripts/ingame-standalone-freecam.sh`) and `ingame-standalone-stratum` take `--game`/`--instance` (use 1.26.60.05 / 1.26.60.05-standalone). Flow: build all mods, make instance, install into `mods/amml`, start exe, poll 180 s for the mod id in the manager log, wait 15 s, end that PID, print logs. Checks: `resolved N/N`, `"missing":0`, `features off: none`, no off/missing/panic/ERROR/WARN, `mod loaded`.
- Mod list hard-coded in `amml/scripts/ingame-mods.sh:7-11` (`repo:task:dist:folder:dll:depinfo`); `check_fresh` fails on a stale DLL.
- For locator-heads: add `../locator-heads:package:dist/alaydriem-locator-heads:alaydriem-locator-heads:alaydriem_locator_heads.dll:target/release/alaydriem_locator_heads.d` to `ingame_mods`; add a task modelled on `ingame-standalone-freecam`; add the recipes and run derive/lint/signature.
- Manual path: build, copy `dist/alaydriem-locator-heads/*` to `...\1.26.60.05-standalone\mods\amml\alaydriem-locator-heads\`, make sure manager and recipes are current, start exe from the instance, read `<instance>\amml\logs\loader.log`, `<instance>\amml\logs\alaydriem-locator-heads.log`, `<instance>\logs\amm\alaydriem-locator-heads\*.log`.
- After any `amml-game`/`amml-sdk` change, rebuild and reinstall every mod (a stale DLL crashed 26.60 once).
- A visual check needs a world with another player. Existing tasks only check logs. Plan a manual join; use `ManagerLink::request_screenshot` or `scripts/capture-game-window.ps1`.

## 6. Open items and risks
1. Render function identity is a lead only: class, vtable start, argument meaning, dot position computation, UI context access still needed.
2. No RTTI for the class: entry must come from a constructor anchor, `xrefs --rva`, or a body pattern.
3. GUI scale and bar rect are not exposed by any entry.
4. Overlay draws above the vanilla frame; the vanilla dot stays unless suppressed or covered.
5. PlayerList and PlayerSkin need new handler-slot entries and skin mirrors.
6. Face extraction (8x8 at (8,8), hat at (40,8), scaled by width/64) belongs in a pure-Rust core crate with tests.
7. Packet hooks on the game thread, frame callback on the render thread: bounded hand-over.
8. `amml/data/*.hcl` is stale; re-run derive before comparing.
9. Verify the LocatorBarPacket handler slot (38).

Task Completed
