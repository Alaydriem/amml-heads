# Locator Heads (26.60) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** An AMML client mod, hosted by the Alaydriem Mod Manager, that draws each player's skin face in place of the dot on the vanilla locator bar in Bedrock 26.60.05.

**Architecture:**
- Packet detours (`PlayerListPacket`, `PlayerSkinPacket`) copy each skin's face and hat into a 32x32 RGBA image. They send it through a bounded channel to the render side.
- A detour on `HudLocatorBarRenderer::render` uploads faces as game textures. It also tags each waypoint's colour with an index.
- A detour on the single-caller icon blit (`0x598a9d0`) decodes the tag and swaps the dot texture for the face texture. Vanilla still computes everything else.

**Tech Stack:**
- Rust 2024 (the toolchain zoom pins)
- `amml-sdk`, `amml-msvc`, `amml-game` (path `../amml`)
- `amm-client`, `amm-core` (path `../modmanager-amml`, branch `amml-port`)
- `crossbeam-channel`, Curia logging, Criterion, cargo-nextest, mise

**Spec:** `docs/superpowers/specs/2026-10-30-locator-heads-design.md`. The evidence is in `docs/findings/{locator-bar-render,skin-pipeline,amml-integration}/findings.md`. Read all four before starting.

## Global Constraints

- Target build: 1.26.60.05, BuildId `6ad79ee4-17d51891a1119914`. Test instance: a private copy, `%LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads`, made from `1.26.60.05-standalone` (the current shim and mod manager). Test only in that copy. Never install into, start, or stop a game in any shared instance: other agents use them. Each game process you start: write its PID to `<instance>\locator-heads.pid` (found by path and creation time with `amml/scripts/find-game-pid.ps1`) and stop only that PID. Never write to `%APPDATA%\levilauncher.exe\versions\`. End only the PID you started (`amml/scripts/find-game-pid.ps1`). Never use `taskkill /IM`.
- **Review policy (operator rule):** no per-task reviewer. Each task's own tests are the gate. One whole-branch review after Task 10.
- **No git commits** (operator rule). Each "Checkpoint" step means: run the listed tests, record the result in the task ledger `docs/findings/plan-ledger/task-N.md`, and end the file with "Task Completed".
- Rules from `amml/CLAUDE.md` and `modmanager-amml/CLAUDE.md` apply to every file:
  - One type per file.
  - No free functions.
  - `new()` constructors.
  - No `unwrap`/`expect` outside tests.
  - `// SAFETY:` on every `unsafe` block.
  - Comments on the preceding line, with no plan references.
  - Tests mirror `src/` under one `integration` target, with no zero-value tests.
  - Curia logging with fields and no format arguments.
  - Bounded collections, each with a test that proves the bound.
- Build and test only through mise tasks. Never pipe build, test or derive output into `grep`, `head` or `tail`.
- Recipes follow `amml/docs/recipes.md`:
  - Two independent recipes and a strong check.
  - Every immediate, displacement and branch byte is wildcarded.
  - Patterns end at a call.
  - `notes` on each entry.
  - `mise run lint` gives 0 open findings.
  - `derive` is unchanged for every existing entry on both builds.
- Mod id `alaydriem-locator-heads`. DLL `alaydriem_locator_heads.dll`. Workspace crates `locator-heads-core` (`heads/core`), `locator-heads-amml` (`heads/amml`, lib name `alaydriem_locator_heads`), `locator-heads-bench` (`heads/bench`).
- Fixed values:

  | Value | Setting |
  | --- | --- |
  | Face image | 32x32 RGBA8 |
  | Face store capacity | 128 |
  | Channel capacity | 256 |
  | Uploads per frame | 4 |
  | Tagged waypoints per frame | 256 |
  | `heads.scale` | 1.0 to 2.0, step 0.25, default 1.0 |
  | `heads.enabled` | default true |

## Review Focus

1. **A player leaves, or their skin changes, between the tag and the blit in one frame.** The blit must draw the vanilla dot or the old face, never a freed texture. Task 8 covers this with the test `remove_during_frame_keeps_texture_until_frame_end`.
2. **A skin that is not RGBA8, has an odd size, or has a short blob** (some servers send 64x32 classic skins or empty persona images). The skin is skipped and the vanilla dot stays. Task 3 covers this with the tests `rejects_rgb8`, `rejects_width_96`, `rejects_short_buffer` and `accepts_64x32`.
3. **The world changes or the player disconnects while faces are loaded.** The store and textures are cleared, and no stale actor id maps a face onto a new world's player. Task 9 covers this with the test `exit_level_clears_store_and_unloads_textures`.
4. **A server colours its waypoints with a colour that looks like a tag.** Because every visible waypoint is tagged in the frame, decode never sees an untagged colour. When there are more than 256 waypoints, nothing is tagged and the blit must pass the colour through. Task 8 covers this with the test `over_capacity_frame_tags_nothing_and_passes_through`.
5. **The game is not 26.60, or an entry is missing.** The mod must not hook anything and must log why. Task 9 covers this with the tests `refuses_when_mesh_size_is_not_2660` and `refuses_when_an_entry_is_missing`.

---

