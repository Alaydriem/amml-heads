# Final review (Opus), 2026-10-30

Verdict: Ready with fixes. No critical finding. No memory-safety defect at the game boundary: every game call was checked against R1-R11 (register order, getTextureGroup hidden return, `this` per call, the 13 blit stack slots, ownership of the ImageBuffer blob and the ResourceLocation string). No double release (SharedPtr has no Drop). Panics are caught; the colours are restored on a panic. `mise run check` passes.

## Important
- I1. The name pool runs out and a face is lost for good (texture_bridge.rs:52, render_state.rs:224-229, face_store.rs:99-101). Released names wait in `deferred` until after_render, but uploads run first in before_render; `next_upload` clears pending before the upload, so a failed upload is never retried. A 129th player, a skin change in a full store, or a world change with 125 or more faces loses faces. Fix: unload `deferred` at the start of upload_pending (after drain); add a FaceStore method that sets pending again on failure; add a test that fills the store and then adds one more face.
- I2. R12 is still open. If the LocatorBarPacket handling ran on another thread, the restore could write into a freed waypoint. Low risk (vanilla walks and erases the list with no lock). Fix: one run that confirms render and player_list log the same thread id.
- I3. The in-game script can damage a live session (ingame-standalone-locator-heads.sh:40,44,51,79): no check for a game already running; it replaces mods and recipes, starts a second game, and deletes the pid file. Fix: FAIL at start when locator-heads.pid exists and pid_alive passes.

## Minor
- M1. RenderState has no Drop, so a disable during a tagged frame leaves tag colours. Fix: Drop restores when the frame is active.
- M2. render_runtime.rs:11-12: the comment says the lock is never held across a game call; before_render and after_render do hold it across game calls. No deadlock (no re-entry; icon() uses try_lock). Fix: correct the comment.
- M3. After a panic or WouldBlock in icon(), the raw tag colour passes through for the rest of the frame. Cosmetic.
- M4. packet_observer.rs:100-117: a skin is extracted even when the channel is full. Fix: return early when sender.is_full(). Drops of Adds over 256 while the HUD is not drawn are an accepted limit.
- M5. format! in two pre-link log lines (heads_mod.rs:85, heads_runtime.rs:181). Acceptable.
- M6. One-recipe field entries fail safe. No action.
- M7. heads_settings.rs:205: a NaN scale passes clamp. Fix: map a non-finite value to MIN_SCALE.
- M8. CLAUDE.md: stale line "Only heads/core exists until...".

## Review Focus 1-5
All met. Item 3 can still lose up to 4 faces on a world change until I1 is fixed.

## Deferred-minor triage
The upload retry must be fixed (part of I1). The M8 line must be fixed. Leave: criterion 0.5, the drop-log cadence, NaN colour packing, textures kept loaded after disable (with M1), ResourceLocation strings leaked at disable.

Task Completed
