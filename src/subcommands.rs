use std::result;

use chrono::{Duration, FixedOffset, Local, TimeZone};

use super::{
    enums,
    errors::{HeliocronError, RuntimeErrorKind},
    report, utils,
};

type Result<T> = result::Result<T, HeliocronError>;

pub fn display_report(report: report::SolarReport) {
    println!("{}", report);
}

pub fn wait(offset: Duration, report: report::SolarReport, event: enums::Event) -> Result<()> {
    let event_time = match event {
        enums::Event::Sunrise => report.sunrise,
        enums::Event::Sunset => report.sunset,
        enums::Event::CivilDawn => report.civil_dawn,
        enums::Event::CivilDusk => report.civil_dusk,
        enums::Event::NauticalDawn => report.nautical_dawn,
        enums::Event::NauticalDusk => report.nautical_dusk,
        enums::Event::AstronomicalDawn => report.astronomical_dawn,
        enums::Event::AstronomicalDusk => report.astronomical_dusk,
    };

    // handle the case when the chosen event doesn't occur on this day
    if event_time.to_string() == "Never" {
        Err(HeliocronError::Runtime(RuntimeErrorKind::NonOccurringEvent))?;
    }

    let wait_until = event_time.datetime.unwrap() + offset;

    let local_time = Local::now();
    let local_time = local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));

    let duration_to_wait = wait_until - local_time;

    utils::wait(duration_to_wait, wait_until)?;
    Ok(())
}
