use std::sync::atomic::{AtomicU32, Ordering};

/// How many more lines of one kind may be logged. A busy server must not fill the log, so each proof line has a
/// budget.
pub struct LineBudget {
    left: AtomicU32,
}

impl LineBudget {
    pub const fn new(lines: u32) -> Self {
        Self { left: AtomicU32::new(lines) }
    }

    /// True while the budget is not spent; each true spends one line.
    pub fn take(&self) -> bool {
        self.left.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |left| left.checked_sub(1)).is_ok()
    }
}
