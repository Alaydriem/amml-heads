use std::sync::atomic::{AtomicBool, Ordering};

/// The one-time info lines that show which thread each detour runs on.
pub struct DetourTrace;

impl DetourTrace {
    /// Logs the thread of the detour `name` the first time it runs; later calls cost one atomic swap.
    pub fn first_run(name: &'static str, ran: &AtomicBool) {
        if !ran.swap(true, Ordering::Relaxed) {
            curia::info!("detour ran for the first time", { detour: name, thread: format!("{:?}", std::thread::current().id()) });
        }
    }
}
