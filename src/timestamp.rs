use std::{
    sync::Mutex,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);

fn ns_since_start() -> i64 {
    let mut start = START_TIME.lock().unwrap();
    match *start {
        Some(start) => {
            let now = Instant::now();
            let ns_since_the_epoch = now.duration_since(start).as_nanos() as i64;
            ns_since_the_epoch
        }
        None => {
            *start = Some(Instant::now());
            0
        }
    }
}

pub fn current_timestamp() -> i64 {
    let now = SystemTime::now();
    let ns_since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64;

    let os = std::env::consts::OS;
    if os == "windows" {
        ns_since_the_epoch + (ns_since_start() % 100)
    } else if os == "macos" {
        ns_since_the_epoch + (ns_since_start() % 1000)
    } else {
        ns_since_the_epoch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_since_start_are_distinct() {
        let mut timestamps = vec![];
        for _ in 0..1000 {
            timestamps.push(ns_since_start());
        }
        for (i, t_i) in timestamps.iter().enumerate() {
            for (j, t_j) in timestamps.iter().enumerate() {
                if i != j {
                    println!("i={}, j={}, t_i={}, t_j={}", i, j, t_i, t_j);
                    assert!(t_i != t_j);
                }
            }
        }
    }

    #[test]
    fn ns_since_start_are_ascending() {
        let mut timestamps = vec![];
        for _ in 0..1000 {
            timestamps.push(ns_since_start());
        }
        for i in 1..timestamps.len() {
            let t_im1 = timestamps[i - 1];
            let t_i = timestamps[i];
            println!("i={}, t_(i-1)={}, t_i={}", i, t_im1, t_i);
            assert!(t_im1 < t_i);
        }
    }

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
