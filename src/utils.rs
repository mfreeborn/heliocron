use std::result;

use chrono::{DateTime, FixedOffset, Local, TimeZone};
use tokio::time::sleep;

use super::errors::{HeliocronError, RuntimeErrorKind};

type Result<T> = result::Result<T, HeliocronError>;

pub(crate) async fn wait(wait_until: DateTime<FixedOffset>) -> Result<()> {
    let local_time = Local::now();
    let local_time = local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));

    let duration_to_wait = wait_until - local_time;

    // Chrono supports negative durations, but std::time::does not. An error here, therefore, tells us that
    // the event occurred in the past.
    let duration_to_wait = match duration_to_wait.to_std() {
        Ok(dur) => Ok(dur),
        Err(_) => Err(HeliocronError::Runtime(RuntimeErrorKind::PastEvent(
            wait_until,
        ))),
    }?;

    println!(
        "Thread going to sleep for {} seconds until {}. Press ctrl+C to cancel.",
        duration_to_wait.as_secs(),
        wait_until
    );
    sleep(duration_to_wait).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "integration-test")]
    #[tokio::test]
    async fn test_wait() {
        use chrono::{FixedOffset, TimeZone};

        use super::*;

        // Some time improbably far in the future.
        let wait_until = FixedOffset::west(0).timestamp(9999999999, 0);
        wait(wait_until).await.unwrap();
    }
}
