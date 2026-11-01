#![allow(unsafe_code)]

use std::sync::atomic::AtomicBool;

use amml_sdk::DetourGuard;
use heads_core::services::QuadScaler;
use heads_core::structs::IconRect;

use crate::layouts::TexturePtrLayout;
use crate::services::{DetourTrace, RenderRuntime};
use crate::structs::IconChoice;

// The locator bar icon blit at 0x598a9d0, called once per waypoint. For a tagged colour it draws the player face in
// white and scaled about its centre; otherwise it draws the vanilla dot in its own colour.
pub static GUARD: DetourGuard = DetourGuard::new("locator_bar_draw_icon");

static RAN: AtomicBool = AtomicBool::new(false);

// this, the render context, the TexturePtr, x, y, u0, u1, v0, v1, w, h, light, alpha, colour, second colour, and
// the two scales; the fourth argument travels in xmm3, the rest from the sixth on are on the stack
pub type DrawIconFn = unsafe extern "C" fn(
    usize,
    usize,
    *const TexturePtrLayout,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    u32,
    u32,
    f32,
    f32,
);

#[allow(clippy::too_many_arguments)]
pub extern "C" fn detour(
    this: usize,
    ctx: usize,
    tex: usize,
    x: f32,
    y: f32,
    u0: f32,
    u1: f32,
    v0: f32,
    v1: f32,
    w: f32,
    h: f32,
    light: f32,
    alpha: f32,
    color: u32,
    color2: u32,
    sx: f32,
    sy: f32,
) {
    let original: DrawIconFn = GUARD.original_fn();
    GUARD.run(
        || {
            DetourTrace::first_run("draw_icon", &RAN);
            let (choice, scale) = RenderRuntime::icon(color);
            match choice {
                IconChoice::Face(face) => {
                    let rect = QuadScaler::scale(IconRect::new(x, y, w, h), scale);
                    // SAFETY: `original` is the next function in the chain, with the hooked signature; `face` is
                    // kept alive by the render state until the frame ends.
                    unsafe {
                        original(
                            this,
                            ctx,
                            face,
                            rect.x,
                            rect.y,
                            u0,
                            u1,
                            v0,
                            v1,
                            rect.w,
                            rect.h,
                            light,
                            alpha,
                            0xFFFF_FFFF,
                            color2,
                            sx,
                            sy,
                        )
                    }
                }
                // SAFETY: as above; the texture is the game's own.
                IconChoice::Vanilla(packed) => unsafe {
                    original(
                        this,
                        ctx,
                        tex as *const TexturePtrLayout,
                        x,
                        y,
                        u0,
                        u1,
                        v0,
                        v1,
                        w,
                        h,
                        light,
                        alpha,
                        packed,
                        color2,
                        sx,
                        sy,
                    )
                },
            }
        },
        // SAFETY: as above, with the arguments unchanged.
        || unsafe {
            original(
                this,
                ctx,
                tex as *const TexturePtrLayout,
                x,
                y,
                u0,
                u1,
                v0,
                v1,
                w,
                h,
                light,
                alpha,
                color,
                color2,
                sx,
                sy,
            )
        },
    );
}
