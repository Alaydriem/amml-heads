use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use heads_core::config::HeadsSettings;

/// The live settings, readable from every thread without a lock.
pub struct SettingsCell {
    enabled: AtomicBool,
    chat: AtomicBool,
    scale_bits: AtomicU32,
}

impl SettingsCell {
    pub fn new(settings: HeadsSettings) -> Self {
        Self {
            enabled: AtomicBool::new(settings.enabled),
            chat: AtomicBool::new(settings.chat),
            scale_bits: AtomicU32::new(settings.scale().to_bits()),
        }
    }

    pub fn store(&self, settings: HeadsSettings) {
        self.enabled.store(settings.enabled, Ordering::Relaxed);
        self.chat.store(settings.chat, Ordering::Relaxed);
        self.scale_bits.store(settings.scale().to_bits(), Ordering::Relaxed);
    }

    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn chat(&self) -> bool {
        self.chat.load(Ordering::Relaxed)
    }

    /// The face scale, already clamped to the supported range.
    pub fn scale(&self) -> f32 {
        f32::from_bits(self.scale_bits.load(Ordering::Relaxed))
    }
}
