/// A feature that `enable` turned off, and the entry that was missing or unusable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnedOff {
    pub feature: &'static str,
    pub entry: String,
}

impl TurnedOff {
    pub fn new(feature: &'static str, entry: impl Into<String>) -> Self {
        Self { feature, entry: entry.into() }
    }
}
