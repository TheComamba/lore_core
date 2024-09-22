use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::*;

static LAST_TIMESTAMP: AtomicI64 = AtomicI64::new(0);

pub fn current_timestamp() -> Timestamp {
    let now = SystemTime::now();
    let mus_since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as i64;

    let mut last_timestamp = LAST_TIMESTAMP.load(Ordering::SeqCst);
    while mus_since_the_epoch <= last_timestamp {
        let new_timestamp = last_timestamp + 1;
        match LAST_TIMESTAMP.compare_exchange_weak(
            last_timestamp,
            new_timestamp,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(_) => return new_timestamp.into(),
            Err(x) => last_timestamp = x,
        }
    }

    LAST_TIMESTAMP.store(mus_since_the_epoch, Ordering::SeqCst);
    mus_since_the_epoch.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_ascending() {
        let mut last_timestamp = current_timestamp();
        for i in 1..1_000_000 {
            let current_timestamp = current_timestamp();
            assert!(
                last_timestamp < current_timestamp,
                "i={}, t_(i-1)={}, t_i={}",
                i,
                last_timestamp,
                current_timestamp
            );
            last_timestamp = current_timestamp;
        }
    }

    #[test]
    fn test_current_timestamp_performance() {
        use std::time::Instant;

        let start = Instant::now();
        for _ in 0..1_000_000 {
            current_timestamp();
        }
        let duration = start.elapsed();

        assert!(
            duration.as_secs_f64() < 1.0,
            "Performance test failed. Duration: {:?}",
            duration
        );
    }
}
