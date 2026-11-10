#![allow(unsafe_code)]

use amml_sdk::amml_detour;

use std::sync::atomic::AtomicBool;

use crate::services::{DetourTrace, HeadsRuntime};

static RAN: AtomicBool = AtomicBool::new(false);

// void LegacyClientNetworkHandler::handle(NetworkIdentifier const&, PlayerSkinPacket const&). The original runs
// first, so a fault while reading the packet never stops vanilla handling; the packet is const& and still alive
amml_detour!(pub mod player_skin(this: usize, source: usize, packet: usize) -> () |original| {
    original(this, source, packet);
    DetourTrace::first_run("player_skin", &RAN);
    if let Some(runtime) = HeadsRuntime::get() {
        // SAFETY: the packet is the game's const& argument, alive until this detour returns
        unsafe { runtime.observe_skin(packet) };
    }
});

pub use player_skin::{GUARD, detour};
