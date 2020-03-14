use chrono::{Duration, FixedOffset, Local, TimeZone};

use super::{enums, report, utils};

pub fn display_report(report: report::SolarReport) {
    println!("{}", report);
}

pub fn wait(offset: Duration, report: report::SolarReport, event: enums::Event) {
    let event_time = match event {
        enums::Event::Sunrise => report.sunrise,
        enums::Event::Sunset => report.sunset,
    };

    let wait_until = event_time + offset;

    let local_time = Local::now();
    let local_time = local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));

    let duration_to_wait = wait_until - local_time;

    utils::wait(duration_to_wait, wait_until);
}
