#![allow(unsafe_code)]

use amml_sdk::amml_detour;

use std::sync::atomic::AtomicBool;

use crate::services::{ChatRuntime, DetourTrace};

static RAN: AtomicBool = AtomicBool::new(false);

// GuiMessage::GuiMessage(MessageType, string const& username, string const& message, optional<string>
// filteredMessage, string const& ttsMessage, string const& authorXuid, float lifetime, bool forceVisible,
// bool ttsRequired). The constructor destroys the filtered message, so after the original only `this` is read
amml_detour!(pub mod gui_message(
    this: usize,
    kind: i32,
    username: usize,
    message: usize,
    filtered_message: usize,
    tts_message: usize,
    author_xuid: usize,
    lifetime: f32,
    force_visible: u8,
    tts_required: u8,
) -> usize |original| {
    let result = original(
        this,
        kind,
        username,
        message,
        filtered_message,
        tts_message,
        author_xuid,
        lifetime,
        force_visible,
        tts_required,
    );
    DetourTrace::first_run("gui_message", &RAN);
    // SAFETY: `this` is the GuiMessage the original just built on this thread; the caller moves it into the chat
    // list only after the constructor returns
    unsafe { ChatRuntime::edit_message(this) };
    result
});

pub use gui_message::{GUARD, detour};
