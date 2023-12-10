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
    fn ns_since_start_are_distinct_and_ascending() {
        let mut timestamps = vec![];
        for _ in 0..1000 {
            timestamps.push(ns_since_start());
        }
        println!("{:?}", timestamps);
        for (i, t_i) in timestamps.iter().enumerate() {
            for (j, t_j) in timestamps.iter().enumerate() {
                if i != j {
                    assert!(t_i != t_j);
                }
                if i < j {
                    assert!(t_i < t_j);
                }
            }
        }
    }

    #[test]
    fn timestamps_are_distinct_and_ascending() {
        let mut timestamps = vec![];
        for _ in 0..1000 {
            timestamps.push(current_timestamp());
        }
        println!("{:?}", timestamps);
        for (i, t_i) in timestamps.iter().enumerate() {
            for (j, t_j) in timestamps.iter().enumerate() {
                if i != j {
                    assert!(t_i != t_j);
                }
                if i < j {
                    assert!(t_i < t_j);
                }
            }
        }
    }
}
