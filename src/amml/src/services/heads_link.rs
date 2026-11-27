use amm_client::link::{EventDrain, LinkState, ManagerLink};
use amm_core::errors::LinkError;
use amm_core::structs::event::ModEvent;
use amml_api::Ctx;
use heads_core::config::{HeadsDocument, HeadsSettings};

use crate::services::HeadsDescriptor;

// the registration with the mod manager; a poll reads at most MAX_EVENTS_PER_POLL events
pub struct HeadsLink {
    link: ManagerLink,
}

impl HeadsLink {
    pub const MAX_DRAINS_PER_POLL: usize = 4;
    pub const MAX_EVENTS_PER_POLL: usize = Self::MAX_DRAINS_PER_POLL * EventDrain::CAPACITY;

    // the link waits for the manager's tables; `state` says when it is connected
    pub fn start(ctx: &Ctx) -> Result<Self, LinkError> {
        let link = ManagerLink::start_without_frame(ctx, HeadsDescriptor::TEXT)?;
        Ok(Self { link })
    }

    pub fn state(&self) -> LinkState {
        self.link.state()
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
