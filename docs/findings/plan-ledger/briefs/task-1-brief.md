### Task 1: Confirm the open reverse-engineering facts (docs only, no product code)

The recipes (Task 2) and the mirrors (Task 7) need numbers that the findings left open. This task measures them on the 26.60.05 exe with `mise run inspect` and `mise run decompile` (in `amml/`). Put the dumps in `docs/findings/re-confirm/`.

**Files:**
- Create: `docs/findings/re-confirm/findings.md`

**Interfaces:**
- Produces: the table below, with every cell filled. Each row gives the RVA or offset, the evidence (dump file and line), and a 26.51 note ("absent", "different", or the value).

| Row | Fact to measure |
| --- | --- |
| R1 | Return type of `0x598a9d0`: does the caller use `rax`/`xmm0` after `0x5746e85`? Do its argument registers and stack slots match `locator-bar-render/findings.md` 2.5? |
| R2 | How the render loop packs colour: `cvttss2si` (truncate) or `cvtss2si` (round), and the clamp order. Give the exact instruction sequence before `0x5746e85`. |
| R3 | `0x3946b0`: which `ResourceLocation` constructor it is (string or `PathView`; is the file system an argument or fixed?). Also the `ResourceLocation` size and its destructor RVA. |
| R4 | `TextureGroup::uploadTexture(ResourceLocation const&, cg::ImageBuffer)`: the RVA. How the by-value `ImageBuffer` is passed (pointer to a caller temp; who destroys it). The `this` adjustment (TextureGroup vs the +0x18 subobject). |
| R5 | `TextureGroup::unloadTexture`: the RVA and `this`. |
| R6 | `cg::ImageBuffer(cg::ImageDescription const&)` constructor RVA. The field order of `cg::ImageDescription` (20 bytes). How to reach the `ImageBuffer` pixel pointer (`mce::Blob` at +0: data pointer offset, size at +0x10). |
| R7 | The `ClientInstance` vslot that returns `shared_ptr<TextureGroup>` (103 by the findings). Check this in the render disasm. |
| R8 | `PlayerListPacket` payload: offset of `mEntries` in the packet, the variant element size, the index byte offset, which index is `AddEntry`, and the `AddEntry` offsets of `mUUID`, `mId` and `mSkin`. Use `inspect fields` on the packet dtor and the 26.60 handler at slot 179. |
| R9 | `PlayerSkinPacket` offsets of `mUUID` and `mSkin` (slot 107 handler). |
| R10 | The `SerializedSkinImpl` offset inside `ThreadOwner<SerializedSkinImpl>`. The offset of `mSkinImage` inside `SerializedSkinImpl`. The `mce::Image` field offsets and the `Blob` data pointer. Read a function that gets an `mce::Image const*` from a `SerializedSkinRef` (a caller of `ClientSkinInfoData::updateSkin`). |
| R11 | `TexturePtr` copy and release: is `amml-msvc` `SharedPtr` enough to add and drop references on the two shared_ptrs, or is a `TexturePtr.dtor` entry needed? |
| R12 | Is the `LocatorBarPacket` handler (slot 38) on the same thread as render? Static evidence only (`ClientInstance` update vs render call chain). Write "unproven" when there is none. Task 10 checks it in game. |

- [ ] **Step 1: Measure R1 to R12** with `mise run inspect -- disasm|fields|xrefs|callers --exe <26.60 exe>` and `mise run decompile`. Each cell needs code evidence. Write "inference" when a cell is inferred.
- [ ] **Step 2: Repeat R3 to R6 and R8 to R10 on the 26.51.01 exe**, only to record whether the layout differs. Do not fix 26.51.
- [ ] **Step 3: Checkpoint.** All cells are filled. Send the file to the operator with SendUserFile. **Stop and report** if R1, R4, R8 or R10 cannot be proven: the draw route or the skin route then needs a design change.

