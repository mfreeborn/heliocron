use std::result;

use chrono::Duration;

use super::{calc, enums, errors, report, utils};

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
    event: enums::Event,
    offset: Duration,
    solar_calculations: calc::SolarCalculations,
    run_missed_task: bool,
) -> Result<()> {
    let event_time = match event {
        enums::Event::SolarNoon => solar_calculations.get_solar_noon(),
        _ => solar_calculations.calculate_event_time(event),
    };

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
