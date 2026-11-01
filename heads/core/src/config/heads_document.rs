use serde::Deserialize;

use super::HeadsSettings;

/// The whole `config.hcl` document; the manager owns every block but `heads`.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(default)]
pub struct HeadsDocument {
    pub heads: HeadsSettings,
}
