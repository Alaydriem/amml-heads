#![allow(unsafe_code)]

use amml_sdk::amml_detour;

use std::sync::atomic::AtomicBool;

use crate::services::{ChatRuntime, DetourTrace, HeadsRuntime};

static RAN: AtomicBool = AtomicBool::new(false);

// bool ClientInstance::update(bool isInitFinished) (26.60: update(bool, bool)): once per frame on the game thread.
// The third argument (r8) is carried as a raw u64 so the register reaches the original bit for bit
amml_detour!(pub mod client_update(this: usize, init_finished: u8, extra: u64) -> u8 |original| {
    let result = original(this, init_finished, extra);
    if let Some(runtime) = HeadsRuntime::get() {
        runtime.tick();
    }
    ChatRuntime::flush_page(this);
    // SAFETY: `this` is the live ClientInstance of this update, on the game thread
    unsafe { ChatRuntime::refresh_font(this) };
    DetourTrace::first_run("client_update", &RAN);
    result
});

pub use client_update::{GUARD, detour};
