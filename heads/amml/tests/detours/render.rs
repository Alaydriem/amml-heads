use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::atomic::Ordering;

use alaydriem_heads::detours::render;
use alaydriem_heads::services::RenderRuntime;
use heads_core::config::HeadsSettings;

use crate::support::{FakeRig, WaypointSpec};

type Seen = (usize, usize, usize, usize, i32, [u8; 4]);

thread_local! {
    static WAYPOINT: Cell<usize> = const { Cell::new(0) };
    static SEEN: RefCell<Vec<Seen>> = const { RefCell::new(Vec::new()) };
}

extern "C" fn fake_original(this: usize, ctx: usize, client: usize, owner: usize, pass: i32) {
    let waypoint = WAYPOINT.with(Cell::get);
    // SAFETY: the test points WAYPOINT at a live fake waypoint, whose colour lanes start at +0x18.
    let lane = unsafe { ((waypoint + 0x18) as *const [u8; 4]).read_unaligned() };
    SEEN.with(|seen| seen.borrow_mut().push((this, ctx, client, owner, pass, lane)));
}

#[test]
fn tags_for_the_original_and_restores_after_it() {
    render::GUARD.original().store(fake_original as *mut c_void, Ordering::Release);
    let rig = FakeRig::new(&[WaypointSpec::plain(Some([0.2, 0.4, 0.6, 1.0]))], HeadsSettings::default());
    let FakeRig { state, waypoints, client, .. } = rig;
    let before = waypoints.bytes(0);
    let waypoint_addr = {
        // the first node's waypoint pointer, read the way the tagger does
        // SAFETY: the fake list head and its first node are live.
        unsafe {
            let first = (waypoints.head() as *const usize).read();
            ((first + 0x20) as *const usize).read()
        }
    };
    WAYPOINT.with(|cell| cell.set(waypoint_addr));
    RenderRuntime::install(state);

    render::detour(1, 2, client.addr(), 4, 5);

    RenderRuntime::uninstall();
    let seen = SEEN.with(|seen| seen.borrow().clone());
    assert_eq!(seen.len(), 1);
    assert_eq!((seen[0].0, seen[0].1, seen[0].2, seen[0].3, seen[0].4), (1, 2, client.addr(), 4, 5));
    // the original saw the tag marker in the first lane, not the waypoint colour
    assert_ne!(seen[0].5, [before[0x18], before[0x19], before[0x1A], before[0x1B]]);
    assert_eq!(waypoints.bytes(0), before);
}

#[test]
fn without_a_runtime_the_original_runs_unchanged() {
    render::GUARD.original().store(fake_original as *mut c_void, Ordering::Release);
    let waypoints = crate::support::FakeWaypoints::new(&[WaypointSpec::plain(Some([0.2, 0.4, 0.6, 1.0]))]);
    let before = waypoints.bytes(0);
    // SAFETY: the fake list head and its first node are live.
    let waypoint_addr = unsafe {
        let first = (waypoints.head() as *const usize).read();
        ((first + 0x20) as *const usize).read()
    };
    WAYPOINT.with(|cell| cell.set(waypoint_addr));

    render::detour(1, 2, 3, 4, 5);

    let seen = SEEN.with(|seen| seen.borrow().clone());
    assert_eq!(seen.len(), 1);
    assert_eq!(waypoints.bytes(0), before);
}
