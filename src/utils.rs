use std::thread;

use chrono::{DateTime, Duration, FixedOffset};

#[inline(never)]
pub fn sleep(duration: Duration, sleep_until: DateTime<FixedOffset>) {
    let duration_to_sleep = match duration.to_std() {
        Ok(dur) => dur,
        Err(_) => panic!("This event has already passed! Must pick a time in the future."),
    };

    println!(
        "Thread going to sleep for {} seconds until {}. Press ctrl+C to cancel.",
        duration_to_sleep.as_secs(),
        sleep_until
    );
    thread::sleep(duration_to_sleep);
}
