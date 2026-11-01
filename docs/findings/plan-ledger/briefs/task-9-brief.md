### Task 9: Mod glue, build gate, manager link and packaging

**Files:**
- Create: `heads/amml/src/heads_mod.rs` (`LocatorHeadsMod: AmmlMod`, `NAME = "alaydriem-locator-heads"`), `heads/amml/src/services/{build_gate.rs, hook_installer.rs, heads_link.rs, descriptor.rs}`, `heads/amml/src/detours/{player_list.rs, player_skin.rs, client_update.rs, exit_level.rs}`
- Create: `heads/amml/amml.hcl`. `entries` lists every Task 2 id plus `ClientInstance.update`, `ClientInstance.getLocalPlayer`, `ClientExitLevel.hook`, `LegacyClientNetworkHandler.vtable` and `Mesh.size`.
- Create: `heads/amml/descriptor.hcl`. It has `claim "Hud" { mode = "Shared", reason = "Replaces the dots on the vanilla locator bar with player faces" }`, and a section `heads` with the fields `heads.enabled` and `heads.scale` (values from Global Constraints, `apply = "Live"`).
- Modify: `mise.toml`, adding the tasks `build-mod` and `package` (dist `dist/alaydriem-locator-heads/`: DLL, PDB, `amml.hcl`).
- Test: `heads/amml/tests/heads_mod.rs`, `heads/amml/tests/services/{build_gate.rs, heads_link.rs}`, `heads/amml/tests/support/{fake_host.rs, mod.rs}` (the zoom fake-host pattern)

**Interfaces:**
- Consumes: everything above. Copy the zoom patterns: `HookInstaller`, linking on the first `ClientInstance::update` (`zoom_runtime.rs:60-75`), and `ManagerLink::without_frame`.
- Produces: the DLL with exactly the four `amml_mod_*` exports.

- [ ] **Step 1: Write failing tests** with a fake host.
  - `refuses_when_mesh_size_is_not_2660`: `Mesh.size` = 0x278. No hook is installed, and one log line has the field `mesh_size`.
  - `refuses_when_an_entry_is_missing`
  - `installs_six_hooks_on_2660`: render, drawIcon, PlayerList, PlayerSkin, ClientInstance.update and ClientExitLevel.
  - `exit_level_clears_store_and_unloads_textures`
  - `settings_change_applies_live`
  - `runs_with_defaults_when_manager_missing`
  - `descriptor_parses_and_has_two_fields`: through `amm-core` descriptor checking, not a serde round trip.
- [ ] **Step 2: Run** `mise run test`. Expected: FAIL.
- [ ] **Step 3: Implement.**
  - `enable` checks the gate, then installs the hooks. The vslot hook targets are read from the vtable globals, as stratum's `table_slot` does.
  - Create the channel (capacity 256) and the `RenderState` (store capacity 128, budget 4).
  - `disable` removes the hooks first, then drops the state.
- [ ] **Step 4: Run** `mise run test`, `mise run check` and `mise run build-mod`. Expected: PASS, clean, and `dist/alaydriem-locator-heads/` with 3 files.
- [ ] **Step 5: Checkpoint.**

