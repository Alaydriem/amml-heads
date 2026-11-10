use std::ffi::c_void;
use std::sync::atomic::AtomicPtr;

// One game hook: the entry of its target, the vtable entry when the target is a slot, the detour written for that
// function's signature, the detour's original slot, and the feature it serves.
#[derive(Clone, Copy)]
pub struct HookSpec {
    pub entry: &'static str,
    // when set, `entry` is a slot in this vtable and the target is the code pointer stored there
    pub table: Option<&'static str>,
    pub detour: *const c_void,
    pub original: &'static AtomicPtr<c_void>,
    pub feature: &'static str,
}
