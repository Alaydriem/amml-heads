### Task 10: In-game verification on 26.60.05 (end-to-end)

**Files:**
- Modify: `amml/scripts/ingame-mods.sh`, adding the line `../locator-heads:package:dist/alaydriem-locator-heads:alaydriem-locator-heads:alaydriem_locator_heads.dll:target/release/alaydriem_locator_heads.d`.
- Create: `amml/scripts/ingame-standalone-locator-heads.sh` and the `ingame-standalone-locator-heads` task in `amml/mise.toml` (a copy of `ingame-standalone-freecam` with `--game` and `--instance`).
- Create: `docs/findings/ingame-2660/findings.md`

- [ ] **Step 1: Add a one-time debug log** (Curia, behind the `trace` level): the thread id the first time each detour runs (R12), the number of faces uploaded, and the number of tag decode misses.
- [ ] **Step 2: Run** `mise run ingame-standalone-locator-heads -- --game <1.26.60.05> --instance <1.26.60.05-locator-heads>`. Before the first run, make the private instance as a copy of `1.26.60.05-standalone`, but not its `mods/amml` folder. Copy only the mod manager into it, then this mod. The script must stop only the PID in `locator-heads.pid`; change the copied script where it does otherwise. Expected:
  - `resolved N/N`
  - `"missing":0`
  - `features off: none`
  - `mod loaded` for `alaydriem-locator-heads`
  - no panic, ERROR or WARN lines
- [ ] **Step 3: Manual check (operator).** The operator joins a world or server with at least 2 other players (one with a classic 64x64 skin, one with a persona skin), with the locator bar on. Take screenshots with `amml/scripts/capture-game-window.ps1`:
  - (a) faces on the bar
  - (b) a player walking behind you: the arrows and fade stay vanilla
  - (c) a skin change: the face updates
  - (d) a player leaves: the icon goes, with no crash
  - (e) `heads.scale` 2.0 in the manager panel
  - (f) world exit and rejoin
  - (g) `heads.enabled` off: vanilla dots
- [ ] **Step 4: Record** the results, the R12 thread ids and the screenshots in `docs/findings/ingame-2660/findings.md`. End the file with "Task Completed". Send it with SendUserFile.
- [ ] **Step 5: Run the end-to-end suites:**
  - `mise run game-test` in `amml/`
  - `mise run e2e` in `modmanager-amml/`
  - the full `mise run test` in all three repos

  Expected: all pass. Report any failure with its output; do not hide it.
- [ ] **Step 6: Checkpoint.** If the work ran in a worktree, mirror `docs/superpowers/` and `docs/findings/` to the root branch under the same paths.
