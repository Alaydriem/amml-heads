#![allow(unsafe_code)]

use std::cell::Cell;
use std::sync::atomic::AtomicBool;

use amml_sdk::DetourGuard;

use crate::services::{DetourTrace, RenderRuntime};

static RAN: AtomicBool = AtomicBool::new(false);

// `HudLocatorBarRenderer::render(MinecraftUIRenderContext&, IClientInstance&, UIControl&, int)`, a virtual slot.
// The waypoint colours are tagged only for the duration of the original call and are put back after it.
pub static GUARD: DetourGuard = DetourGuard::new("locator_bar_render");

// this, the render context, the client, the owning control and the pass
pub type RenderFn = unsafe extern "C" fn(usize, usize, usize, usize, i32);

pub extern "C" fn detour(this: usize, ctx: usize, client: usize, owner: usize, pass: i32) {
    let original: RenderFn = GUARD.original_fn();
    let called = Cell::new(false);
    GUARD.run(
        || {
            RenderRuntime::before(client);
            called.set(true);
            // SAFETY: `original` is the next function in the chain, with the hooked signature; the arguments are the
            // game's own.
            unsafe { original(this, ctx, client, owner, pass) };
            RenderRuntime::after();
            DetourTrace::first_run("render", &RAN);
        },
        || {
            // a panic may have left the colours tagged
            RenderRuntime::recover();
            if !called.get() {
                // SAFETY: as above.
                unsafe { original(this, ctx, client, owner, pass) };
            }
        },
    );
}
