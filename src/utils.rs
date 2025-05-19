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
    let duration_to_wait = duration_to_wait
        .to_std()
        .map_err(|_| HeliocronError::Runtime(RuntimeErrorKind::PastEvent(wait_until)))?;

    println!(
        "Thread going to sleep for {} seconds until {}. Press ctrl+C to cancel.",
        duration_to_wait.as_secs(),
        wait_until
    );
    sleep(duration_to_wait).await;
    Ok(())
}
