use std::result;

use chrono::{Duration, Local};

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

pub fn display_now(sc: calc::SolarCalculations, civil: bool) -> Result<()> {
    // Get times for sunrise/sunset or dawn/dusk
    let report = report::SolarReport::new(sc);

    let (begin, end) = match if civil {
        (report.civil_dawn.datetime, report.civil_dusk.datetime)
    } else {
        (report.sunrise.datetime, report.sunset.datetime)
    } {
        (Some(begin), Some(end)) => (begin, end),
        (Some(_), None) => panic!("Missing value for dusk"),
        (None, Some(_)) => panic!("Missing value for dawn"),
        (None, None) => panic!("Missing values for dawn and dusk"),
    };

    // Is the current time between these two?
    let now = Local::now();

    if begin <= now && now <= end {
        println!("day");
    } else {
        println!("night");
    }

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

    match event_time.datetime {
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
