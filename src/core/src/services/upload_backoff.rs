/// Spaces out the retries of an upload that keeps failing. The first `FAILURES_BEFORE_BACKOFF - 1` failures in a
/// row retry on the next frame; from then on, a retry runs once every `RETRY_FRAMES` frames. A success resets it.
pub struct UploadBackoff {
    failures: u32,
    wait: u32,
    reported: bool,
}

impl UploadBackoff {
    pub const FAILURES_BEFORE_BACKOFF: u32 = 3;
    pub const RETRY_FRAMES: u32 = 64;

    pub fn new() -> Self {
        Self { failures: 0, wait: 0, reported: false }
    }

    /// Called once a frame: whether this frame may try the upload.
    pub fn ready(&mut self) -> bool {
        if self.wait > 0 {
            self.wait -= 1;
            return false;
        }
        true
    }

    /// Records a failed upload. True only for the first failure in the life of this back-off, so it is logged once.
    pub fn failed(&mut self) -> bool {
        self.failures = self.failures.saturating_add(1);
        if self.failures >= Self::FAILURES_BEFORE_BACKOFF {
            self.wait = Self::RETRY_FRAMES - 1;
        }
        let first = !self.reported;
        self.reported = true;
        first
    }

    /// Records a successful upload, or a change that makes the next try worth running at once.
    pub fn reset(&mut self) {
        self.failures = 0;
        self.wait = 0;
    }
}

impl Default for UploadBackoff {
    fn default() -> Self {
        Self::new()
    }
}
