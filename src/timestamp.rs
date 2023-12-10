use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

static LAST_TIMESTAMP: Mutex<i64> = Mutex::new(0);

pub fn current_timestamp() -> i64 {
    let now = SystemTime::now();
    let mut ns_since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64;
    let mut last_timestamp = LAST_TIMESTAMP.lock().unwrap();
    while *last_timestamp >= ns_since_the_epoch {
        ns_since_the_epoch = ns_since_the_epoch + 1;
    }
    *last_timestamp = ns_since_the_epoch;
    ns_since_the_epoch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_distinct() {
        let mut timestamps = vec![];
        for _ in 0..1000 {
            timestamps.push(current_timestamp());
        }
        for (i, t_i) in timestamps.iter().enumerate() {
            for (j, t_j) in timestamps.iter().enumerate() {
                if i != j {
                    assert!(t_i != t_j);
                }
            }
        }
    }

    #[test]
    fn timestamps_are_ascending() {
        let mut timestamps = vec![];
        for _ in 0..1000 {
            timestamps.push(current_timestamp());
        }
        for i in 1..timestamps.len() {
            let t_im1 = timestamps[i - 1];
            let t_i = timestamps[i];
            println!("i={}, t_(i-1)={}, t_i={}", i, t_im1, t_i);
            assert!(t_im1 < t_i);
        }
    }
}
