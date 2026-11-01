use std::ffi::c_void;
use std::path::Path;
use std::sync::atomic::Ordering;

use alaydriem_heads::detours::exit_level;
use alaydriem_heads::services::{ChatRuntime, ChatState, HeadsRuntime, PacketObserver};
use alaydriem_heads::structs::PacketOffsets;
use crossbeam_channel::bounded;
use heads_core::services::GlyphPage;
use heads_core::structs::FaceUpdate;

extern "C" fn fake_original(_this: usize) {}

#[test]
fn leaving_the_world_sends_clear_and_runs_the_original() {
    exit_level::GUARD.original().store(fake_original as *const () as *mut c_void, Ordering::Release);
    let runtime = HeadsRuntime::install(Path::new("."));
    let (sender, receiver) = bounded(8);
    runtime.set_observer(PacketObserver::new(sender, PacketOffsets::new(0xD8, 0xD0, 0xA8, 0x80, 0xF8)));

    exit_level::detour(0x88);

    assert_eq!(receiver.try_recv().unwrap(), FaceUpdate::Clear);
}

#[test]
fn leaving_the_world_offers_the_chat_page_for_upload_again() {
    exit_level::GUARD.original().store(fake_original as *const () as *mut c_void, Ordering::Release);
    ChatRuntime::install(ChatState::new(GlyphPage::new(&[]), ChatRuntime::flags()), ChatRuntime::flags());
    ChatRuntime::flush_page_with(|_| true);

    exit_level::detour(0x88);

    let mut offered = false;
    ChatRuntime::flush_page_with(|_| {
        offered = true;
        true
    });
    assert!(offered);
}
