use std::result;

use chrono::{DateTime, Duration, FixedOffset};

use super::errors::{HeliocronError, RuntimeErrorKind};

use tokio_walltime;

type Result<T> = result::Result<T, HeliocronError>;

async fn sleep(time: DateTime<FixedOffset>) -> Result<()> {
    if cfg!(feature = "integration-test") || cfg!(test) {
        println!("Fake sleep until {}s.", time);
    } else {
        tokio_walltime::sleep_until(time).await?;
    }
    Ok(())
}

pub async fn wait(duration: Duration, wait_until: DateTime<FixedOffset>) -> Result<()> {
    let duration_to_wait = match duration.to_std() {
        Ok(dur) => Ok(dur),
        Err(_) => Err(HeliocronError::Runtime(RuntimeErrorKind::PastEvent)),
    }?;

    println!(
        "Thread going to sleep for {} seconds until {}. Press ctrl+C to cancel.",
        duration_to_wait.as_secs(),
        wait_until
    );
    sleep(wait_until).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    #[tokio::test]
    async fn test_wait() {
        let duration_to_wait = Duration::seconds(5);
        let wait_until = FixedOffset::west(0).timestamp(9999999999, 0);
        wait(duration_to_wait, wait_until).await.unwrap();
    }
}
