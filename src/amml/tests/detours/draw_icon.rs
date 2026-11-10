use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::atomic::Ordering;

use alaydriem_heads::detours::draw_icon;
use alaydriem_heads::layouts::TexturePtrLayout;
use alaydriem_heads::services::RenderRuntime;
use heads_core::config::HeadsSettings;

use crate::support::{FakeGame, FakeRig, WaypointSpec};

#[derive(Debug, Clone, PartialEq)]
struct Call {
    this: usize,
    ctx: usize,
    tex: usize,
    floats: [f32; 12],
    color: u32,
    color2: u32,
}

thread_local! {
    static CALLS: RefCell<Vec<Call>> = const { RefCell::new(Vec::new()) };
}

#[allow(clippy::too_many_arguments)]
extern "C" fn fake_original(
    this: usize,
    ctx: usize,
    tex: *const TexturePtrLayout,
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
    let floats = [x, y, u0, u1, v0, v1, w, h, light, alpha, sx, sy];
    CALLS.with(|calls| calls.borrow_mut().push(Call { this, ctx, tex: tex as usize, floats, color, color2 }));
}

fn install_fake_original() {
    draw_icon::GUARD.original().store(fake_original as *mut c_void, Ordering::Release);
}

#[allow(clippy::too_many_arguments)]
fn call_detour(tex: usize, x: f32, y: f32, w: f32, h: f32, color: u32) {
    draw_icon::detour(11, 22, tex, x, y, 0.1, 0.9, 0.2, 0.8, w, h, 1.0, 0.5, color, 0xFFFF_FFFF, 1.0, 1.0);
}

#[test]
fn untagged_frame_forwards_every_argument_in_order() {
    install_fake_original();

    call_detour(0x77, 10.0, 20.0, 7.0, 8.0, 0xFF12_3456);

    let calls = CALLS.with(|calls| calls.borrow().clone());
    assert_eq!(
        calls,
        vec![Call {
            this: 11,
            ctx: 22,
            tex: 0x77,
            floats: [10.0, 20.0, 0.1, 0.9, 0.2, 0.8, 7.0, 8.0, 1.0, 0.5, 1.0, 1.0],
            color: 0xFF12_3456,
            color2: 0xFFFF_FFFF,
        }]
    );
}

#[test]
fn face_swaps_texture_and_colour_and_scales_about_the_centre() {
    install_fake_original();
    FakeGame::take_log();
    let rig =
        FakeRig::new(&[WaypointSpec::player(42, Some([1.0, 0.0, 0.0, 1.0]))], HeadsSettings::new(true, 2.0, true));
    rig.add_face(1, 42, [9, 8, 7, 6]);
    let FakeRig { state, waypoints, client, .. } = rig;
    RenderRuntime::install(state);
    RenderRuntime::before(client.addr());

    call_detour(0x77, 10.0, 20.0, 7.0, 7.0, waypoints.current_packed(0));

    RenderRuntime::after();
    RenderRuntime::uninstall();
    let calls = CALLS.with(|calls| calls.borrow().clone());
    assert_eq!(calls.len(), 1);
    assert_ne!(calls[0].tex, 0x77);
    assert_ne!(calls[0].tex, 0);
    assert_eq!(calls[0].color, 0xFFFF_FFFF);
    assert_eq!(&calls[0].floats[..2], &[6.5, 16.5]);
    assert_eq!(&calls[0].floats[6..8], &[14.0, 14.0]);
    assert_eq!(&calls[0].floats[2..6], &[0.1, 0.9, 0.2, 0.8]);
}

#[test]
fn vanilla_icon_in_a_tagged_frame_gets_its_own_colour_back() {
    install_fake_original();
    let rig = FakeRig::new(&[WaypointSpec::plain(Some([0.5, 0.25, 0.999, 1.0]))], HeadsSettings::default());
    let FakeRig { state, waypoints, client, .. } = rig;
    RenderRuntime::install(state);
    RenderRuntime::before(client.addr());

    call_detour(0x77, 10.0, 20.0, 7.0, 7.0, waypoints.current_packed(0));

    RenderRuntime::after();
    RenderRuntime::uninstall();
    let calls = CALLS.with(|calls| calls.borrow().clone());
    assert_eq!(calls[0].tex, 0x77);
    assert_eq!(calls[0].color, 0xFF7F_3FFE);
    assert_eq!(&calls[0].floats[..2], &[10.0, 20.0]);
}
