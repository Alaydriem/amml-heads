# Locator Heads: design (Bedrock 26.60)

Date: 2026-10-30. Status: draft for review.
Evidence: `docs/findings/locator-bar-render/findings.md`, `docs/findings/skin-pipeline/findings.md`, `docs/findings/amml-integration/findings.md`.

## Intent

On the vanilla locator bar, show each player as their skin face (face layer plus hat layer) instead of a coloured dot. It must work in any world and on any server. It uses only the skin that the client already gets over the network. It changes nothing on the server and sends no packets.

## Goals

- Bedrock 26.60.05 Windows client (BuildId `6ad79ee4-17d51891a1119914`), standalone AMML host. This is the only build that is tested.
- The face is drawn by the game's own locator bar draw call, so its position, smoothing, culling, fade, sorting and up and down arrows stay vanilla.
- The face uses the skin from the `PlayerListPacket` and `PlayerSkinPacket` that the client receives. A skin change is shown on the bar.
- An AMM client mod: it has a descriptor, settings in the manager panel and per-mod logs. It also runs with default settings when the manager is not present.
- Memory has a hard limit, and frame cost is near zero (no allocation per frame).

## Non-goals

- 26.51 and later builds. The recipes and mirrors are written for 26.60. The mod refuses to enable on any other layout.
- Persona or custom geometry skins whose head UV is not at the classic place. These show whatever is at the classic face UV. v1 does not parse geometry.
- Waypoints that are not players: script waypoints, location waypoints, and entity waypoints for actors that are not in the player list. These keep the vanilla icon.
- A coloured outline, player names or distance text on the bar.
- The LeviLamina host. It may work, but it is not tested.

## Anti-goals (the mod fails even if a face appears)

- The game crashes or freezes, or the vanilla bar breaks, when a skin is malformed, a player leaves while their icon is drawn, or the world changes.
- A face appears on the wrong player.
- The mod writes game memory and leaves it changed after the draw (the waypoint colour must always be restored).

## Decisions

1. **Draw route (hook A of the render findings).**
   - Detour `HudLocatorBarRenderer::render` (vtable `0xF260170` slot 16, `0x5745690`).
   - Detour the icon blit `0x598a9d0`, which has one caller, in render.
   - Before the original render runs, the render detour writes a unique tag colour into each visible waypoint's `mColor`. After it runs, the detour restores every colour.
   - The blit detour decodes the packed colour argument back to a waypoint index.
     - When the index is a player that has a face, the detour passes the face `TexturePtr` and white, and keeps vanilla alpha.
     - Otherwise it passes the waypoint's own vanilla colour, packed exactly as vanilla packs it.
2. **Waypoint key.**
   - `ClientEntityWaypoint` (vtable `0xF13DFD0`) has an ActorUniqueID at +0x78.
   - That ID matches `PlayerListPacket` `AddEntry.mId`.
   - Plain `ClientWaypoint`s are never given a face.
3. **Skin source.**
   - Detour `LegacyClientNetworkHandler::handle` for `PlayerListPacket` (slot 179) and `PlayerSkinPacket` (slot 107).
   - Read `SerializedSkinImpl::mSkinImage` from the packet, then copy and compose the face on the game thread.
   - Only the finished face image (32x32 RGBA, 4096 bytes) crosses to the render side, through a bounded channel.
4. **Face image.**
   - Take the 8x8 face at (8,8) and the 8x8 hat at (40,8), scaled by `width / 64`.
   - Blend the hat over the face with source-over alpha.
   - Upscale with nearest neighbour to 32x32, so that linear sampling stays sharp.
   - Accept only RGBA8 (`ImageFormat` 4) with a width of 64, 128 or 256, and a height equal to the width or half of it.
5. **Game texture.**
   - On the render thread, inside the render detour, upload at most 4 faces per frame. Use `cg::ImageBuffer`, then `TextureGroup::uploadTexture` with a mod-owned `ResourceLocation` (`locator_heads/<actor id hex>_<generation>`), then `TextureGroup::getTexture` to get a `TexturePtr`.
   - When a face is evicted or replaced, unload it with `TextureGroup::unloadTexture`.
6. **Bounds.**
   - The face store holds 128 faces with LRU eviction. The update channel holds 256 messages, and the newest message is dropped when it is full.
   - Each frame tags at most 256 waypoints. When a frame has more, nothing is tagged that frame and all icons stay vanilla.
7. **Threads.**
   - Packet detours only send to the channel.
   - The render detour owns the store and the textures (`try_lock` on one process-global state, which is never contended by design).
   - The blit detour reads the per-frame tag table that the render detour fills.
8. **Build gate.**
   - Every entry the mod uses is required.
   - The mod also refuses to enable unless `Mesh.size == 0x260` (26.60 layout). This stays until another build is tested.
9. **Manager.**
   - The descriptor id is `alaydriem-locator-heads`.
   - It has a `Hud` claim in `Shared` mode.
   - Settings:
     - `heads.enabled` (bool, default true, Live)
     - `heads.scale` (float 1.0 to 2.0, step 0.25, default 1.0, Live). The quad is scaled around its centre.
   - It links on the first `ClientInstance.update`, as zoom does.
10. **Repos.**
    - The mod lives in `locator-heads/`, a new workspace that copies the shape of zoom.
    - The new recipes go in `amml/recipes/`.
    - Entry ids are mod-local `EntryId::new` constants, as stratum does. No `amml-sdk` or `amml-game` API changes.
    - Nothing is committed (operator rule).

## Left to the builder

- Exact recipe patterns. They follow `amml/docs/recipes.md` (two independent recipes and a strong check).
- Internal module names below the files the plan names.
