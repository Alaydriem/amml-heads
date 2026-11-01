use heads_core::services::UploadBackoff;

fn frames_until_ready(backoff: &mut UploadBackoff) -> u32 {
    let mut frames = 1;
    while !backoff.ready() {
        frames += 1;
    }
    frames
}

#[test]
fn the_first_two_failures_retry_on_the_next_frame() {
    let mut backoff = UploadBackoff::new();
    assert!(backoff.ready());
    backoff.failed();
    assert!(backoff.ready());
    backoff.failed();
    assert!(backoff.ready());
}

#[test]
fn from_the_third_failure_a_retry_runs_every_64_frames() {
    let mut backoff = UploadBackoff::new();
    backoff.failed();
    backoff.failed();
    backoff.failed();
    assert_eq!(frames_until_ready(&mut backoff), UploadBackoff::RETRY_FRAMES);
    backoff.failed();
    assert_eq!(frames_until_ready(&mut backoff), UploadBackoff::RETRY_FRAMES);
}

#[test]
fn a_success_ends_the_back_off() {
    let mut backoff = UploadBackoff::new();
    for _ in 0..UploadBackoff::FAILURES_BEFORE_BACKOFF {
        backoff.failed();
    }
    backoff.reset();
    assert!(backoff.ready());
    backoff.failed();
    assert!(backoff.ready());
}

#[test]
fn only_the_first_failure_is_reported() {
    let mut backoff = UploadBackoff::new();
    assert!(backoff.failed());
    assert!(!backoff.failed());
    backoff.reset();
    assert!(!backoff.failed());
}
