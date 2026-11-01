# Locator bar render: how the dots are drawn (Bedrock 26.60, Windows)

- Build: `6ad79ee4-17d51891a1119914` (the 1.26.60.05 instance exe, read only). All addresses are RVAs.
- Tags: [proven] = read from decompiled code or disassembly. [header] = LeviLamina 26.60 headers. [inference] = reasoned guess, not checked in code.
- Lead check: the call at `0x5746e85 call 0x598a9d0`, the `mov r13,[r13+0xf00]` list read at `0x5745dc3` and the two arrow calls at `0x5747052` / `0x57470cc` were checked again in `disasm_5745690_render.txt`.
- Note: the amml-integration findings guessed two vtables at `0xf260100` / `0xf260190`. That split was wrong. Both functions are in one vtable at `0xF260170` (`0xF260188` = slot 3, `0xF2601F0` = slot 16).

## Summary
- `HudLocatorBarRenderer` has no RTTI. Found from the texture strings that its update loads. [proven]
  - Vtable: `0xF260170`. render(MinecraftUIRenderContext&, IClientInstance&, UIControl&, int): `0x5745690`, slot 16. update: `0x5747be0`, slot 3.
- Waypoint data comes from `LocalPlayer::mClientLocatorBar` at LocalPlayer+0xEF0: an `unordered_map<WaypointHandle (mce::UUID), unique_ptr<ClientWaypoint>>`. render reads the list head pointer at LocalPlayer+0xF00. `PlayerData` and `ActorUniqueIDCompare` are not used in render. [proven]
- Each dot uses the waypoint's own `mTexturePath`, sent by the server (players: `textures/ui/locator_bar_dot_N`). render loads it with `TextureGroup::getTexture` (`0x541f580`) and draws it with one call to `0x598a9d0` (icon blit). `0x598a9d0` has exactly one caller in the exe: the locator bar render (at `0x5746e85`). [proven]
- `mTextures[0..1]` / `mMeshes[0..1]` are the up and down arrows, drawn with `0xe9b010`. `mTextures[2..5]` / `mMeshes[2..5]` (dot_0..dot_3) are loaded and built but never drawn by render. [proven]
- A player waypoint is a `ClientEntityWaypoint` (vtable `0xF13DFD0`). Its ActorUniqueID is at +0x78. `tryGetActorID` is slot 7. [proven]

## 1. Header diff 26.51 -> 26.60
About 35 headers diffed (every LocatorBar / Waypoint file, plus HudLocatorBarRenderer, HudPlayerRenderer, MinecraftUIRenderContext).
- HudLocatorBarRenderer.h: `mMeshes` 3792 -> 3648 bytes (`mce::Mesh` 0x278 -> 0x260). `mBackgroundSize` 0xFB0 -> 0xF20, `mLastRenderCall` 0xFB8 -> 0xF28. Earlier members and signatures unchanged.
- MinecraftUIRenderContext.h: `TextItem.mText` string (32) -> `variant<string, string_view>` (40); `mShadow`, `mShowColorSymbol` move to the end of `TextItem`. A new virtual `drawText(..., std::string_view, ...)` overload moves all later UIRenderContext vtable slots by one. Members, `ImageItem`, `drawImage`, `getTexture`, `flushImages` unchanged.
- ServerWaypoint.h: `$dtor` MCFOLD -> MCAPI. No layout change.
- No change: all ClientLocatorBar / ClientWaypoint* headers (empty stubs in both), Waypoint, WaypointGroup, LocatorBar packet and payload headers, VanillaWaypointManager*, ServerLocatorBar, ServerEntityWaypoint, ServerPlayerWaypoint, HudPlayerRenderer, LocalPlayer members.
- Related: `BaseActorRenderContext::Impl::mOffscreenCaptureDescription` 40 -> 32; `getCameraRotation` and `getFrameAlpha(Actor const&)` removed. `ClientSkinInfoData` adds `mLodSkinProfileHash`; `setLodSkinImage` -> `trySetLodSkinImage`. `TextureGroup::shouldLoadPBRResources` removed.

## 2. render and update in the 26.60 exe

### 2.1 How they were found [proven]
- `locator_bg` (0xF5B8953), `locator_arrow_up` (0xF572D66), `locator_arrow_down` (0xF57FB44) are loaded only by `0x5747be0`. The four dot paths are inlined copies inside it: 0xF7E3479, 0xF7DE056, 0xF7D995D, 0xF7D6504.
- `0x5747be0` has one data reference, at `0xF260188` = slot 3 of the vtable at `0xF260170`. Constructor `0x57447a0` and destructor `0x576cce0` both `lea` that vtable.
- Forwarders: slot 5 (`0x465ae0`) is `jmp [vtbl+0x80]` -> slot 16; slot 4 -> slot 15; slot 1 -> slot 17.
- Slots: 0 dtor `0x576cce0`; 1 preRenderSetup `0x575a8f0`; 2 clone `0x57455d0`; 3 update `0x5747be0`; 4 frameUpdate `0x2395d50`; 5 render(UIRenderContext&) `0x465ae0`; 15 frameUpdate (MC context) `0x768f0` empty; 16 render (MC context) `0x5745690`; 17 preRenderSetup (MC context) `0x768f0` empty.
- Object layout agrees with the header: +0x08 float alpha (probably `mPropagatedAlpha` [inference]); +0x10 `mBackgroundTexture`; +0x18 `mHasLoadedIconTextures`; +0x20 `mTextures` (stride 0x20); +0xE0 `mMeshes` (stride 0x260); +0xF20 `mBackgroundSize`; +0xF28 `mLastRenderCall`.

### 2.2 update (`0x5747be0`) [proven]
- rcx=this, rdx=IClientInstance&, r8=UIControl& owner, r9=UIScene const&. Returns al=1.
- Rebuilds the locator_bg nine-patch when null or when the owner size (owner+0x48/+0x4C) changes by more than FLT_EPSILON.
- When `mHasLoadedIconTextures` is 0, loads all six textures once with `TextureGroup::getTexture` on group+0x18 (TextureGroupBase subobject). Order: [0] arrow_up, [1] arrow_down, [2] dot_0, [3] dot_1, [4] dot_2, [5] dot_3.

### 2.3 render (`0x5745690`) [proven unless tagged]
Entry: rcx=this, rdx=MinecraftUIRenderContext*, r8=IClientInstance*, r9=UIControl* owner, [rsp+0x28]=int pass (unused).
1. Player and gates: player = client vslot 30 (0xF0) = getLocalPlayer (matches AMML `ClientInstance.getLocalPlayer = 0x1E`). Gates `0x5741750` and `0x57354b0`, each (client, player, true); if either is true, render returns (probably HUD-hidden checks [inference]). Returns when `mBackgroundTexture` is null.
2. Bar rectangle: barW = owner+0x48, barH = owner+0x4C, barX = owner+0x10, barY = owner+0x14. When owner+0x18 & 1 is set, `0x1c68360` recomputes the position. screenCtx = *(renderCtx+0x10).
3. Lazy mesh build: for each invalid mesh, `0xe9ad70(&mMeshes[i], &mTextures[i], screenCtx+0xB0 tessellator, &Vec2)`. With m = max(0, barH-3): arrows (barH+2, barH-1); dot_0 and dot_1 5+m; dot_2 3+m; dot_3 1+m. Returns if any texture is not loaded yet.
4. Background: shader color at screenCtx+0x30 = {1,1,1,alpha}, then the nine-patch (`0x1913870`).
5. View direction: client bool (vslot 50) selects the player view vector (`0xba31b0`, negated when `0xaa6f60` is true) or a camera forward vector at (client vslot 202 result)+0x118. Player position from `0xbcda70` (interpolated position [inference]).
6. Waypoint list: head = *(player+0xF00); walks node->next until head. `ClientWaypoint* = *(node+0x20)`; key at node+0x10 is the UUID handle. Sorts the pointers by squared distance (comparator `0x5767d20`; direction not checked).
7. Per waypoint `wp`:
   - wp+0x60 (`mIsVisible`) == 0: clears wp+0x74 and skips.
   - Smoothing: position at wp+0x68 (`optional<Vec3>`, flag +0x74) is lerped toward wp+0x08 with factor 1-exp(-k*dt), k = 20 - 18*clamp((d^2-256)/3840, 0, 1); dt from +0xF28.
   - Horizontal angle: yaw delta wrapped to (-pi, pi]; drawn only when |yaw| < pi/3.
   - Needs wp+0x50 == 1 (`mTexturePath` has a value).
   - Texture group: client vslot 103 (0x338) returns `shared_ptr<TextureGroup>` (through MinecraftGame vslot 10). Null group stops the whole loop.
   - The string at wp+0x30 becomes a ResourceLocation (`0x3946b0`); `0x541f580` gets the texture; empty texture skips the waypoint.
   - Color: wp+0x18 (`optional<Color>`, flag +0x28), or white when empty; clamped to [0,1], *255, packed 0xAARRGGBB.
   - Size: w = iconSize.x*5 + m, h = iconSize.y*5 + m (iconSize at wp+0x58/+0x5C).
   - Position: cx = barX + barW/2 + (yaw/(pi/3))*0.5*(barW - (barH + w + 2)); cy = barY + barH/2; top-left (cx - w/2, cy - h/2).
   - Context: `BaseActorRenderContext` on the stack, `0x57e1da0(&ctx, screenCtx, client, IMinecraftGame from client vslot 78)`.
   - Dot: `call 0x598a9d0` at `0x5746e85`.
   - Arrows: pitch compared with ±acos(...)*0.5*0.85; below: `0xe9b010(screenCtx, &mMeshes[1], &mTextures[1], &{1, &Vec3{x,y,0}}, alpha, 0)` at `0x57470cc`; above: same with [0] at `0x5747052`. The arrow bobs 0.5 or 1.5 units with time.
   - Destroys the context (`0x57e2220`).

### 2.4 ClientLocatorBar and waypoint layout [proven]
- `handle(LocatorBarPacket)` = ClientNetworkHandler vtable `0xF2CF5B0` slot 38 = `0x6ba1ec0`. It calls `0x1e8a280(getLocalPlayer()+0xEF0, packet)`.
- In `0x1e8a280`: hash = (hi*0x1F1F1F1F ^ lo) & mask; bucket vector at this+0x20, mask at this+0x38, list head at this+0x10, erase on this+8. So `ClientLocatorBar` = {8 bytes; unordered_map at +8} = 72 bytes (header agrees).
- Payload stride 0x98 (Payload 0x80, Handle at +0x80, Action at +0x90). On Add with flags 0x3F: if `mActorID` has a value -> new `ClientEntityWaypoint` (0x98 bytes, vtable `0xF13DFD0`) with actor id at +0x78; else -> new `ClientWaypoint` (0x78 bytes, vtable `0xF114660`).
- `ClientWaypoint` layout: 0x00 vtable; 0x08 WorldPosition (Vec3; dim at +0x14); 0x18 `optional<Color>` (flag 0x28); 0x30 `optional<string>` (flag 0x50); 0x58 iconSize; 0x60 visible; 0x61 clientPositionAuthority; 0x68 `optional<Vec3>` smoothed position (flag 0x74).
- `ClientEntityWaypoint` adds ActorUniqueID at +0x78 and an entity reference at +0x80. Its update (`0x25147e0`) follows the actor.
- `tryGetActorID` (slot 7, `0x25149f0`): `mov rax,rdx; mov rcx,[rcx+78h]; mov [rdx],rcx; mov byte [rdx+8],1; ret`. Base version (`0x83620`) returns nullopt.

### 2.5 Calling conventions
- Dot blit `0x598a9d0` [proven arguments; names inference]:
  - rcx = *(ctx+0x60) (`mHudIconActorRenderer` by the header [inference]; uses a material at this+0x168)
  - rdx = BaseActorRenderContext*
  - r8 = `mce::TexturePtr const*` (32 bytes: two shared_ptrs)
  - xmm3 = x (left); [rsp+0x20] = y (top)
  - [rsp+0x28] u0 = 0, [rsp+0x30] u1 = 1, [rsp+0x38] v0 = 0, [rsp+0x40] v1 = 1
  - [rsp+0x48] = w, [rsp+0x50] = h
  - [rsp+0x58] = 1.0 (light); [rsp+0x60] = alpha (renderer this+8)
  - [rsp+0x68] = packed color 0xAARRGGBB; [rsp+0x70] = 0xFFFFFFFF (second color)
  - [rsp+0x78] = 1.0, [rsp+0x80] = 1.0 (scales)
  - Emits vertices (u0,v1), (u1,v1), (u1,v0), (u0,v0); each UV clamped to [0,1].
- Arrow draw `0xe9b010` (19 callers): rcx=ScreenContext&, rdx=Mesh*, r8=TexturePtr*, r9=&{u64 count, Vec3* positions}, [rsp+0x20]=float alpha, [rsp+0x28]=0.
- `TextureGroup::getTexture` `0x541f580` (159 callers): rcx=TextureGroup+0x18, rdx=out TexturePtr*, r8=ResourceLocation*, r9b=forceReload, [rsp+0x20]=optional<uint>, [rsp+0x28]=TextureSetLayerType.
- `BaseActorRenderContext`: ctor `0x57e1da0`(rcx=this, rdx=ScreenContext&, r8=IClientInstance&, r9=IMinecraftGame&); dtor `0x57e2220`.
- Client vslots: 30 = getLocalPlayer; 50 = a bool; 78 = IMinecraftGame ([ci+0x1C8]); 103 = TextureGroup; 202 = ci+0x378 (camera data). Names are inference except slot 30.

## 3. The 6 textures and meshes [proven]
| i | Texture | Path | Mesh | Size | Drawn by render |
|---|---|---|---|---|---|
| 0 | +0x20 | locator_arrow_up | +0xE0 | barH+2 x barH-1 | yes |
| 1 | +0x40 | locator_arrow_down | +0x340 | barH+2 x barH-1 | yes |
| 2 | +0x60 | locator_bar_dot_0 | +0x5A0 | 5+m | no |
| 3 | +0x80 | locator_bar_dot_1 | +0x800 | 5+m | no |
| 4 | +0xA0 | locator_bar_dot_2 | +0xA60 | 3+m | no |
| 5 | +0xC0 | locator_bar_dot_3 | +0xCC0 | 1+m | no |

- Order matches `ImageType` (Arrowup, Arrowdown, PlayerDot0..3). The server picks the dot tier (by distance [inference]) and sends it as `mTexturePath`; the client draws whatever path it gets. `0x2990370` or `0x2990560` is probably `getPathFromPlayerDotImageType` [inference].

## 4. Drawing a custom image: API and precedent
- MinecraftUIRenderContext [header]: `drawImage(ClientTexture const&, vec2 pos, vec2 size, vec2 uv, vec2 uvSize, bool)` then `flushImages(Color, alpha, HashedString material)`; `getTexture(ResourceLocation, bool) -> TexturePtr`, `getZippedTexture`, `unloadTexture`, `getUITextureInfo`, `touchTexture`, `drawNineslice`, `fill/drawRectangle`. `mScreenContext` at +0x10 (proven). Slots not resolved; 26.60 slots after `drawText` move by one.
- Face texture routes [header + inference]:
  - Upload with `TextureGroup::uploadTexture(ResourceLocation, cg::ImageBuffer)`, then fetch with the same `getTexture` the bar uses.
  - A `TexturePtr` is {`shared_ptr<BedrockTextureData const>`, `shared_ptr<ResourceLocation>`}. It could wrap `ClientSkinInfoData::mSkinTexture.mBedrockTextureData` (ClientSkin component; rendered actors only). The blit reads only *TexturePtr and copies the two shared_ptrs [proven].
  - Raw pixels: `SerializedSkinImpl::mSkinImage`. Face (8,8)-(16,16), hat (40,8)-(48,16) on a 64-wide skin.
- Precedent [header; not decompiled]: `ProfileImageRenderer` (`shared_ptr<mce::Image>` + `mce::ClientTexture`, image loaded at run time). `ActorPortraitRenderer`, `HudPlayerRenderer`, `PaperDollRenderer` draw 3D models.

## 5. Recommended hook points
**A (recommended): detour the dot blit `0x598a9d0`.**
- One caller, so no side effects elsewhere.
- For a known player: replace r8 with the face TexturePtr, set the color to white. Otherwise call the original unchanged.
- Vanilla still computes position, smoothing, culling, sort, alpha and arrows.
- The callee does not get the waypoint. Two ways to find it:
  1. Color tag: in a render detour (slot 16, `0x5745690`), before the original, save `mColor` (+0x18..+0x28) of each waypoint and write a unique tag color. In the blit detour, decode arg 14. Restore the colors after the original returns.
  2. Caller frame: caller [rbp+0x570] holds the iterator to the `ClientWaypoint*` (caller rbp = entry rsp + 8 + 0x80). Build-specific; needs a recipe (at `0x5746b98` / `0x5746cd6`) and an asm thunk.
- Cost: two hooked entries, one with a 17-argument signature.

**B: change the texture path, no draw hook.** In a render pre-detour set `mTexturePath` (+0x30) of each target to a custom path, color white; upload the face once with `TextureGroup::uploadTexture`; restore after. One hook, but writes a `std::string` in game memory every frame, and `cg::ImageBuffer` is not mirrored yet.

**C: overlay after render.** Walk *(LocalPlayer+0xF00), use wp+0x68 and the formulas in 2.3; hide the vanilla dot by clearing wp+0x50 before the original and restoring after (also removes arrows). Must copy the view-yaw branches; can drift; still needs a draw route.

Ranking: A with color tag, then B, then C.

## Open items (not proven)
- Thread of render and of packet handling (both expected on the main thread).
- Meaning of client vslots 50 and 202, and of the two render gates.
- Sort direction of the draw list.
- Whether a TexturePtr built from an uploaded texture renders through `0x598a9d0`.
- Real 26.60 slots of `drawImage` / `flushImages`.
- Owner class of the blit.

## Dumps in this folder
fn_5745690_HudLocatorBarRenderer_render.c, disasm_5745690_render.txt, fn_5747be0_HudLocatorBarRenderer_update.c, disasm_5747be0_update.txt, fn_598a9d0_dot_blit.c, fn_e9b010_render_mesh_instanced.c, fn_e9ad70_build_quad_mesh.c, fn_541f580_TextureGroup_getTexture.c, fn_57e1da0_BaseActorRenderContext_ctor.c, fn_57e2220_BaseActorRenderContext_dtor.c, fn_6ba1ec0_handle_LocatorBarPacket.c, fn_1e8a280_ClientLocatorBar_handlePacket.c, fn_25147e0_ClientEntityWaypoint_update.c, fn_5741750_render_gate1.c, fn_57354b0_render_gate2.c.

Task Completed
