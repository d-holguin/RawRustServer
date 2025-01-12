use std::{
    sync::atomic::AtomicUsize,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
    pub session_id: String,
    pub last_active: Instant,
}

impl Session {
    pub fn generate_session_id() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Failed to generate session id");

        let counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!(
            "{}{}{}",
            since_the_epoch.as_secs(),
            since_the_epoch.subsec_nanos(),
            counter
        )
    }
}
