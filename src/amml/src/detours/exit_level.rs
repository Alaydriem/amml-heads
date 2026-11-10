use amml_sdk::amml_detour;

use std::sync::atomic::AtomicBool;

use crate::services::{ChatRuntime, DetourTrace, HeadsRuntime};

static RAN: AtomicBool = AtomicBool::new(false);

// void ClientInstance::onLevelExit(): every face and texture goes before the original runs, so no actor id maps a
// face onto a player of the next world, and the chat glyph page is uploaded again after it. `this` is the
// GameCallbacks subobject, passed on unchanged
amml_detour!(pub mod exit_level(this: usize) -> () |original| {
    if let Some(runtime) = HeadsRuntime::get() {
        runtime.clear_faces();
    }
    ChatRuntime::on_world_exit();
    original(this);
    DetourTrace::first_run("exit_level", &RAN);
});

pub use exit_level::{GUARD, detour};
