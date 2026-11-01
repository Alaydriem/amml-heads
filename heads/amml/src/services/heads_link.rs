use std::path::Path;

use amm_client::link::{ApiTable, EventDrain, ManagerLink};
use amm_core::errors::LinkError;
use amm_core::structs::event::ModEvent;
use heads_core::config::{HeadsDocument, HeadsSettings};

use super::HeadsDescriptor;

// the registration with the mod manager; a poll reads at most MAX_EVENTS_PER_POLL events
pub struct HeadsLink {
    link: ManagerLink,
}

impl HeadsLink {
    pub const MAX_DRAINS_PER_POLL: usize = 4;
    pub const MAX_EVENTS_PER_POLL: usize = Self::MAX_DRAINS_PER_POLL * EventDrain::CAPACITY;

    pub fn new(table: ApiTable, config_dir: &Path) -> Result<Self, LinkError> {
        let link = ManagerLink::without_frame(table, HeadsDescriptor::TEXT, config_dir)?;
        Ok(Self { link })
    }

    pub fn settings(&self) -> HeadsSettings {
        match self.link.config::<HeadsDocument>() {
            Ok(document) => document.heads,
            Err(error) => {
                curia::warn!("config could not be read; using the defaults", { error: error.to_string() });
                HeadsSettings::default()
            }
        }
    }

    // reads the queued manager events; true when the settings changed
    pub fn poll(&mut self) -> bool {
        let mut changed = false;
        for _ in 0..Self::MAX_DRAINS_PER_POLL {
            let mut read = 0;
            for event in self.link.events() {
                read += 1;
                changed |= matches!(event, ModEvent::SettingsChanged { .. });
            }
            if read < EventDrain::CAPACITY {
                break;
            }
        }
        changed
    }
}
