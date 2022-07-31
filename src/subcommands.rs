use std::result;

use chrono::{Duration, Local};

use super::{calc, domain, errors, report, utils};

type Result<T> = result::Result<T, errors::HeliocronError>;

pub fn display_report(solar_calculations: calc::SolarCalculations, json: bool) -> Result<()> {
    let report = report::SolarReport::new(solar_calculations);
    let output = if json {
        serde_json::to_string(&report).unwrap()
    } else {
        report.to_string()
    };
    println!("{}", output);
    Ok(())
}

pub async fn wait(
    event: domain::Event,
    offset: Duration,
    solar_calculations: calc::SolarCalculations,
    run_missed_task: bool,
) -> Result<()> {
    let event_time = solar_calculations.event_time(event);

    match event_time.0 {
        Some(datetime) => {
            let wait_until = datetime + offset;
            utils::wait(wait_until).await?;

            // If the device running heliocron is asleep for whetever reason, it is possible that this future
            // will return after `wait_until`. As such, we need to handle whether to run or skip the task
            // if the event was missed. We allow a default tolerance of 30s, which should be more than enough to
            // catch any scheduling delays that could cause a second or two's delay. At some point, this arbitrary
            // number could be made configurable, if desired.
            if run_missed_task {
                Ok(())
            } else {
                let now = chrono::Utc::now().with_timezone(wait_until.offset());
                let missed_by = (now - wait_until).num_seconds();
                if missed_by > 30 {
                    Err(errors::HeliocronError::Runtime(
                        errors::RuntimeErrorKind::EventMissed(missed_by),
                    ))
                } else {
                    Ok(())
                }
            }
        }
        None => Err(errors::HeliocronError::Runtime(
            errors::RuntimeErrorKind::NonOccurringEvent,
        )),
    }
}

pub fn poll(solar_calculations: calc::SolarCalculations) -> Result<()> {
    let now = Local::now();
    let report = report::SolarReport::new(solar_calculations);

    if let (Some(sunrise), Some(sunset)) = (report.sunrise.0, report.sunset.0) {
        if now >= sunrise && now < sunset {
            println!("day");
            return Ok(());
        }
    };

    if let (Some(civil_dawn), Some(civil_dusk)) = (report.civil_dawn.0, report.civil_dusk.0) {
        if now >= civil_dawn && now < civil_dusk {
            println!("civil_twilight");
            return Ok(());
        }
    };

    if let (Some(nautical_dawn), Some(nautical_dusk)) =
        (report.nautical_dawn.0, report.nautical_dusk.0)
    {
        if now >= nautical_dawn && now < nautical_dusk {
            println!("nautical_twilight");
            return Ok(());
        }
    };

    if let (Some(astronomical_dawn), Some(astronomical_dusk)) =
        (report.astronomical_dawn.0, report.astronomical_dusk.0)
    {
        if now >= astronomical_dawn && now < astronomical_dusk {
            println!("astronomical_twilight");
            return Ok(());
        } else {
            println!("night");
            return Ok(());
        }
    };

    // TODO: to get here is an error
    Ok(())
}
