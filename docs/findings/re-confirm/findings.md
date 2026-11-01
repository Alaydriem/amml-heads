# Task 1: reverse-engineering facts confirmed (26.60.05, BuildId `6ad79ee4-17d51891a1119914`)

All 12 rows are filled. R1, R4, R8 and R10 are proven from code, so the draw route and the skin route need no design change. Two cells are not proven: the R3 destructor RVA (not needed) and R12 (Task 10 checks it in game).
Dumps are in this folder. "LBR" = `docs/findings/locator-bar-render/`. Addresses are RVAs.

| Row | 26.60 value | Evidence | 26.51 note |
| --- | --- | --- | --- |
| R1 | `0x598a9d0` returns void; args match LBR findings 2.5 | LBR `disasm_5745690_render.txt` L1257-1293; `disasm_598a9d0_dot_blit.txt` L21-25, L60-67; `disasm_598b0e1_dot_blit_tail.txt` L46-59 | not measured |
| R2 | clamp max(0) then min(1), *255, `cvttps2dq` (truncate), `0xAARRGGBB` summed with paddd | LBR render L1258-1279; constants `0xEF3FDE0`, `0xF0827E0`, `0xF0827F0`, `0xF082800` | not measured |
| R3 | `0x3946b0` = `ResourceLocation(Core::PathView)`, fs fixed 0 (UserPackage), size 0x38; dtor unproven (MCFOLD, inlined as a free of mPath) | `fn_3946b0_ResourceLocation_ctor.c`; `disasm_3946b0` L10-12; LBR render L1190-1205 | ctor `0x2709a0` |
| R4 | `uploadTexture(ResourceLocation const&, cg::ImageBuffer)` = `0x54204f0`; this = full TextureGroup*; r8 = caller temp; callee frees the blob | `disasm_54204f0_uploadTexture_ImageBuffer.txt` L8-23, L41-46; caller `disasm_64db900` L18-30 | `0x1d0df40`, same shape |
| R5 | `unloadTexture` = `0x541d8b0`; rcx = TextureGroup*, rdx = loc, r8b unused; returns al = found | `disasm_541d8b0_unloadTexture.txt` L10-38; `disasm_64db900` L81-82 | `0x1d0b340` |
| R6 | `ImageBuffer(ImageDescription const&)` = `0x5a4bb80`; ImageDescription w@0 h@4 fmt@8 (RGBA8 = 0x1C) colorSpace@0xC imageType@0xD alphaUsage@0xE depth@0x10; ImageBuffer (0x30): deleter@0, data@8, size@0x10, desc@0x18 | `disasm_5a4bb80` L10, L24-28, L51; `fn_5a4bb80.c`; `fn_5a4bd30.c`; `fn_5a4c000.c` | `0x5964d10` |
| R7 | ClientInstance slot 103 (`[vtbl+0x338]`) -> out `shared_ptr<TextureGroup>`; `add r14,0x18` before getTexture | LBR render L1136-1144, L1181 | not measured |
| R8 | packet+0x30 `mEntries` (begin/end/cap); element 0xD8; index byte +0xD0 (0 Remove, 1 Add); AddEntry mUUID +0x08, mId +0x18, mSkin +0xA8 (ctrl +0xB0); handler = LegacyClientNetworkHandler vtable `0xF2CFE40` slot 179 = `0x6bdd410` | `disasm_6bdd410` L175-212; `fn_4045d50_AddEntry_toPlayerListEntry.c` L16-134; `vtable_F2CFE40` L181 | different: element 0xB8, index +0xB0, mSkin +0x88; handler `0x1348a10` slot 171 |
| R9 | PlayerSkinPacket (0x98): mUUID +0x30, mSkin +0x40 (ctrl +0x48); handler slot 107 = `0x6bdec10` | `fn_6bdec10_handle_PlayerSkinPacket.c` L22-25, L43, L98-108 | same offsets; handler `0x134a4c0` slot 100 |
| R10 | SerializedSkinImpl at +0 of ThreadOwner; mSkinImage at impl+0x80; mce::Image format@0 w@4 h@8 depth@0xC usage@0x10 alphaUsage@0x11 Blob@0x18 (deleter +0x18, data +0x20, size +0x28). Pixels = *(impl+0xA0), size = *(impl+0xA8) | `fn_298f4e0_SerializedSkinRef_size.c` L14-23; `fn_6bdee80.c`; `fn_5a4c000.c` | different: mSkinImage at impl+0xA0 (removed mPlayFabId) |
| R11 | amml-msvc `SharedPtr` is enough; TexturePtr = {SharedPtr<BedrockTextureData> @0, SharedPtr<ResourceLocation> @0x10}; no dtor entry | `disasm_598a9d0` L303-349; `disasm_598b0e1` L5-19 | dtor MCFOLD on both |
| R12 | inference: packet handler and render on one thread (no lock on either side); static proof unproven | LBR `fn_1e8a280` and render | not measured |

## Notes

- **R1.**
  - Call site:
    - rcx = [rbp+0x68] (ctx+0x60)
    - rdx = &ctx (rbp+8)
    - r8 = &TexturePtr (rbp+0x4B0)
    - xmm3 = x
  - Stack slots:

    | Slot | Value |
    | --- | --- |
    | 0x20 | y |
    | 0x28 | u0 = 0 |
    | 0x30 | u1 = 1 |
    | 0x38 | v0 = 0 |
    | 0x40 | v1 = 1 |
    | 0x48 | w |
    | 0x50 | h |
    | 0x58 | 1.0 |
    | 0x60 | alpha ([rdi+8]) |
    | 0x68 | colour |
    | 0x70 | 0xFFFFFFFF |
    | 0x78 | 1.0 |
    | 0x80 | 1.0 |

  - r9, xmm1 and xmm2 carry nothing.
  - The callee takes a separate path when colour == 0xFFFFFFFF (`0x598ab60`), and a no-texture path when [r8] is null (`0x598af7b`).
  - The return is void: rax is not set, and the caller overwrites xmm0 at `0x5746e93` before any read.
- **R2.**
  - The exact sequence is: `xorps xmm1,xmm1; maxps xmm1,[rbp+0x530]; movaps xmm2,[0xEF3FDE0]; minps xmm2,xmm1; mulps xmm2,[0xF0827E0]; cvttps2dq xmm1,xmm2`.
  - It then runs pshufd/pmuludq with the weights, punpckldq, pshufd/paddd x2, and `movd [rsp+0x68],xmm1`.
  - A NaN channel gives 0x80000000 in its lane.
  - The source is wp+0x18 `optional<Color>`. When it is empty, white comes from `0xF0FEAA0`.
- **R3.**
  - Arguments: rcx = out, rdx = PathView* {char const* data; u64 size}. It returns rcx.
  - Layout:
    - +0 fs (u32)
    - +8 std::string mPath
    - +0x28 mPathHash = FNV-1a-64 of the path (0 when empty)
    - +0x30 mFullHash = ((fs ^ 0xcbf29ce484222325) * 0x100000001b3) ^ pathHash
  - To destroy it, free mPath with MsvcString on the game heap. No entry is needed.
- **R4.**
  - Signature: `BedrockTexture& (TextureGroup*, ResourceLocation const*, ImageBuffer*)`.
  - The body converts the buffer to a TextureContainer (`0x4DF8EB0`), then calls the 3-argument overload `0x5420EC0`.
  - At the end, the callee frees the blob: if [r8+8] != 0, it calls [r8](data). The caller must not destroy the buffer again.
  - Precedent caller `0x64DB900`:
    - It builds the buffer with `0x5A4C000` (ImageBuffer(mce::Image&&)), then moves it with `0x5A4BD30`.
    - It uploads, then calls getTexture(tg+0x18, out, loc, r9d = 0, [rsp+0x20] = 0, [rsp+0x28] = 0).
    - It unloads with r8d = 0.
- **R5.**
  - The body finds loc in the map at +0x1A8 and tells the LRU at +0x240.
  - When found, it calls `0x5676FE0` on the node's BedrockTexture (+0x58), then erases loc from the map at +0x230.
  - It does NOT erase the node from +0x1A8 (inference). So every distinct location name that is ever uploaded leaves a node behind.
- **R6.**
  - For a face, use the description {w, h, 0x1C, colorSpace 1 (SRGB, the addEmptyTexture default), imageType 0, alphaUsage 0, depth 1}.
  - The ctor allocates bpp*w*h*depth bytes through `0x3C7480`, and the deleter is `0x3C7B10`. The mod overwrites the pixels.
- **R8.** AddEntry offsets:

  | Field | Offset |
  | --- | --- |
  | name | +0x20 |
  | xuid | +0x40 |
  | playFabId | +0x60 |
  | platformOnlineId | +0x80 |
  | buildPlatform | +0xA0 |
  | bools | +0xB8..+0xBA |
  | colour | +0xBC |

  - The handlers are in the LegacyClientNetworkHandler vtable. The ClientNetworkHandler vtable holds the empty `0x768F0` at slots 107 and 179.
- **R9.** The handler checks isValid(+0x40) with `0x6BDEE80`, then runs the skin memory gate `0x6BC4800`.
- **R10.** Check format == 4 and size == w*h*4 before reading the pixels.
- **R11.**
  - Add a reference with lock inc [ctrl+8] on each control block, which is clone_ref of both SharedPtrs.
  - Release each control block: lock dec [ctrl+8], then vtbl[0]; lock dec [ctrl+0xC], then vtbl[1].
  - Release +0x10 first, then +0.

Task Completed
