use chrono::{DateTime, Duration, FixedOffset, Local, NaiveTime, TimeZone};

pub fn parse_date(
    date: Option<&str>,
    date_fmt: &str,
    time_zone: Option<&str>,
) -> DateTime<FixedOffset> {
    // default date format
    let time_fmt = "%H:%M:%S";
    let datetime_fmt = format!("{}T{}", date_fmt, time_fmt);

    // customisable date
    // e.g. 2020-02-24
    let date = match date {
        Some(d) => d.to_string(),
        None => Local::today().format(date_fmt).to_string(),
    };
    let time = "12:00:00";
    let datetime = format!("{}T{}", date, time);

    // default time zone is the local time zone at the given date
    let time_zone = match time_zone {
        Some(tz) => tz.to_string(),
        None => Local
            .datetime_from_str(&datetime, &datetime_fmt)
            .expect("Error parsing date!")
            .offset()
            .to_string(),
    };
    let datetimetz = format!("{}{}", datetime, time_zone);
    let datetimetz_fmt = format!("{}%:z", datetime_fmt);

    DateTime::parse_from_str(&datetimetz, &datetimetz_fmt).expect("Error parsing date!")
}

pub fn parse_latlon(latlon: &str) -> f64 {
    // W and S should be negative
    let compass_direction: &str = &latlon[latlon.len() - 1..].to_lowercase();

    let compass_correction = match compass_direction {
        "n" | "e" => 1.0,
        "w" | "s" => -1.0,
        _ => panic!("Expected latitude/longitude to end with one of: N, S, E, W"),
    };

    let latlon: f64 = latlon[..latlon.len() - 1]
        .parse()
        .expect("Error, float expected!");

    latlon * compass_correction
}

pub fn parse_event(event: &str) -> String {
    let event = match event {
        "sunrise" | "sunset" => event,
        _ => panic!("--event argument must be one of: 'sunrise' | 'sunset'"),
    };
    event.to_string()
}

pub fn parse_offset(offset: &str) -> Duration {
    // offset should either be %H:%M:%S or %H:%M
    let offset = match offset {
        offset if NaiveTime::parse_from_str(offset, "%H:%M:%S").is_ok() => {
            NaiveTime::parse_from_str(offset, "%H:%M:%S").unwrap()
        }
        offset if NaiveTime::parse_from_str(offset, "%H:%M").is_ok() => {
            NaiveTime::parse_from_str(offset, "%H:%M").unwrap()
        }
        _ => panic!("Error parsing offset! Expected the format to be one of: %H:%M:%S | %H:%M"),
    };
    offset.signed_duration_since(NaiveTime::from_hms(0, 0, 0))
}
