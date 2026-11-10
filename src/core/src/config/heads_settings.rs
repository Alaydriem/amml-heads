use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct HeadsSettings {
    #[serde(default = "HeadsSettings::default_enabled")]
    pub enabled: bool,
    #[serde(default = "HeadsSettings::default_scale")]
    pub scale: f32,
    #[serde(default = "HeadsSettings::default_chat")]
    pub chat: bool,
}

impl HeadsSettings {
    pub const MIN_SCALE: f32 = 1.0;
    pub const MAX_SCALE: f32 = 2.0;

    pub fn new(enabled: bool, scale: f32, chat: bool) -> Self {
        Self { enabled, scale, chat }
    }

    /// The face scale, clamped to the supported range. A value that is not finite counts as the smallest scale.
    pub fn scale(&self) -> f32 {
        if !self.scale.is_finite() {
            return Self::MIN_SCALE;
        }
        self.scale.clamp(Self::MIN_SCALE, Self::MAX_SCALE)
    }

    fn default_enabled() -> bool {
        true
    }

    fn default_scale() -> f32 {
        1.0
    }

    fn default_chat() -> bool {
        true
    }
}

impl Default for HeadsSettings {
    fn default() -> Self {
        Self::new(Self::default_enabled(), Self::default_scale(), Self::default_chat())
    }
}
