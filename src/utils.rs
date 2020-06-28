use std::result;

use chrono::{DateTime, Duration, FixedOffset};

use super::errors::{HeliocronError, RuntimeErrorKind};

type Result<T> = result::Result<T, HeliocronError>;

fn sleep(dur: std::time::Duration) {
    if cfg!(feature = "integration-test") || cfg!(test) {
        println!("Fake sleep for {}s.", dur.as_secs());
    } else {
        std::thread::sleep(dur);
    };
}

pub fn wait(duration: Duration, wait_until: DateTime<FixedOffset>) -> Result<()> {
    let duration_to_wait = match duration.to_std() {
        Ok(dur) => Ok(dur),
        Err(_) => Err(HeliocronError::Runtime(RuntimeErrorKind::PastEvent)),
    }?;

    println!(
        "Thread going to sleep for {} seconds until {}. Press ctrl+C to cancel.",
        duration_to_wait.as_secs(),
        wait_until
    );
    sleep(duration_to_wait);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    #[test]
    fn test_wait() {
        let duration_to_wait = Duration::seconds(5);
        let wait_until = FixedOffset::west(0).timestamp(9999999999, 0);
        wait(duration_to_wait, wait_until).unwrap();
    }
}
