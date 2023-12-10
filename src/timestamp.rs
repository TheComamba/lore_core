use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> i64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_nanos() as i64
}
