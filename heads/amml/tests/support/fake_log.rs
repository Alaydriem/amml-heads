use std::sync::{Mutex, PoisonError};

use curia::{Dispatch, Level, LogEvent, Logger};

static EVENTS: Mutex<Vec<LogEvent>> = Mutex::new(Vec::new());

/// One error line a test captured: the fixed text and the two fields that name the feature and the entry.
#[derive(Debug, PartialEq, Eq)]
pub struct ErrorLine {
    pub message: String,
    pub feature: String,
    pub entry: String,
}

/// A Curia dispatcher that keeps every event in memory. Each test runs in its own process, so the one global
/// dispatcher is the test's own.
pub struct FakeLog;

impl FakeLog {
    pub fn install() {
        let _ = Logger::install(Box::new(Self));
    }

    pub fn errors() -> Vec<ErrorLine> {
        let events = EVENTS.lock().unwrap_or_else(PoisonError::into_inner);
        events
            .iter()
            .filter(|event| event.level == Level::Error)
            .map(|event| ErrorLine {
                message: event.message.clone(),
                feature: Self::field(event, "feature"),
                entry: Self::field(event, "entry"),
            })
            .collect()
    }

    fn field(event: &LogEvent, key: &str) -> String {
        event.fields.get(key).and_then(|value| value.as_str()).unwrap_or_default().to_string()
    }
}

impl Dispatch for FakeLog {
    fn dispatch(&self, event: &LogEvent) {
        EVENTS.lock().unwrap_or_else(PoisonError::into_inner).push(event.clone());
    }
}
