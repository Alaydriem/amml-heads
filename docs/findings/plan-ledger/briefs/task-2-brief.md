### Task 2: Recipes for the new entries (in `amml/`)

**Files:**
- Create: `amml/recipes/hud_locator_bar_renderer.hcl`, `amml/recipes/texture_group.hcl`, `amml/recipes/resource_location.hcl`, `amml/recipes/image_buffer.hcl`, `amml/recipes/serialized_skin.hcl`, `amml/recipes/player_list_packet.hcl`, `amml/recipes/player_skin_packet.hcl`
- Modify: `amml/recipes/client_network_handler.hcl` (two vslot entries), `amml/recipes/local_player.hcl` (one field entry), `amml/recipes/client_instance.hcl` (one vslot entry)

**Interfaces:**
- Consumes: the Task 1 table.
- Produces: these entry ids, used by Tasks 7 to 9.

| Entry id | Kind | 26.60 value | Notes |
| --- | --- | --- | --- |
| `HudLocatorBarRenderer.vtable` | global | `0xF260170` | Recipes: the `lea` in ctor `0x57447a0` and in dtor `0x576cce0` |
| `HudLocatorBarRenderer.render` | vslot | 16 | Check `in_vtable_of`, range 14..18 |
| `HudLocatorBarRenderer.drawIcon` | function, `hooked = true` | `0x598a9d0` | Recipes: body pattern, and `ref` render then `call` at the blit site |
| `LocalPlayer.mClientLocatorBar` | field | `0xEF0` | Recipes: the LocatorBarPacket handler `add`/`lea`, and the render `+0xF00` read minus `0x10` |
| `ClientInstance.getTextureGroup` | vslot | R7 | |
| `ClientNetworkHandler.handle.PlayerListPacket` | vslot | 179 | Same shape as `...ContainerOpenPacket` |
| `ClientNetworkHandler.handle.PlayerSkinPacket` | vslot | 107 | |
| `TextureGroup.getTexture` | function | `0x541f580` | |
| `TextureGroup.uploadTexture` | function | R4 | |
| `TextureGroup.unloadTexture` | function | R5 | |
| `ResourceLocation.ctor` | function | R3 | |
| `ResourceLocation.dtor` | function | R3 | |
| `ImageBuffer.ctorFromDescription` | function | R6 | |
| `PlayerListPacket.entries` | field | R8 | Also `PlayerListAddEntry.size`, `.mUUID`, `.mId`, `.mSkin`, `PlayerListPacket.variantIndex` |
| `PlayerSkinPacket.mUUID`, `PlayerSkinPacket.mSkin` | field | R9 | |
| `SerializedSkinRef.impl`, `SerializedSkinImpl.mSkinImage` | field | R10 | |

  Use one field entry per offset that moved between 26.51 and 26.60 ("one name, one layout"). Offsets that the headers show as stable may be constants in the mirrors, with an `offset_of!` assert.

- [ ] **Step 1: For each entry, run `mise run inspect -- draft <rva>`**, wildcard the pattern, and check `inspect scan` gives one match. Write the entry with two independent recipes, a check and `notes`.
- [ ] **Step 2: Run** `mise run derive -- --exe <26.60 exe> --recipes recipes --out <tmp> --report <tmp> --compare data/6ad79ee4-17d51891a1119914.hcl`. Expected: every new entry resolves to its table value, and no existing value moves. Run it twice; the two outputs must be identical.
- [ ] **Step 3: Run the same derive on the 26.51.01 exe.** Expected: no existing value moves. New entries either refuse or resolve; record each in the ledger. A wrong 26.51 value is allowed only because the build gate (Task 9) refuses 26.51.
- [ ] **Step 4: Run** `mise run lint -- --exe <exe> --recipes recipes --report <file>` on both builds. Expected: 0 open findings. Run `mise run signature -- --exe <26.60 exe>`. Expected: `HudLocatorBarRenderer.drawIcon` is listed with the R1 registers.
- [ ] **Step 5: Checkpoint.** `mise run check` and `mise run test` in `amml/` pass.

