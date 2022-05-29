use std::result;

use chrono::{Duration, FixedOffset, Local, TimeZone};

use super::{calc, enums, errors, report, utils};

type Result<T> = result::Result<T, errors::HeliocronError>;

pub fn display_report(solar_calculations: calc::SolarCalculations) -> Result<()> {
    let report = report::SolarReport::new(solar_calculations);
    println!("{}", report);
    Ok(())
}

pub async fn wait(
    event: enums::Event,
    offset: Duration,
    solar_calculations: calc::SolarCalculations,
) -> Result<()> {
    let event_time = match event {
        enums::Event::SolarNoon => solar_calculations.get_solar_noon(),
        enums::Event::Sunrise { .. }
        | enums::Event::Sunset { .. }
        | enums::Event::CivilDawn { .. }
        | enums::Event::CivilDusk { .. }
        | enums::Event::NauticalDawn { .. }
        | enums::Event::NauticalDusk { .. }
        | enums::Event::AstronomicalDawn { .. }
        | enums::Event::AstronomicalDusk { .. }
        | enums::Event::CustomAM { .. }
        | enums::Event::CustomPM { .. } => solar_calculations.calculate_event_time(event),
    };

    match event_time.datetime {
        Some(datetime) => {
            let wait_until = datetime + offset;

            let local_time = Local::now();
            let local_time =
                local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));

            let duration_to_wait = wait_until - local_time;

            utils::wait(duration_to_wait, wait_until).await?;
        }
        None => {
            Err(errors::HeliocronError::Runtime(
                errors::RuntimeErrorKind::NonOccurringEvent,
            ))?;
        }
    };
    Ok(())
}
