use std::str::FromStr;

use chrono::{DateTime, Duration, FixedOffset, Local, NaiveTime, TimeZone};

use super::enums::Event;

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

pub fn parse_event(event: &str) -> Event {
    Event::from_str(event).expect(&format!(
        "Error parsing event. Expected one of {{sunrise | sunset}}, got \"{}\".",
        event
    ))
}

pub fn parse_offset(offset: &str) -> Duration {
    // offset should either be %H:%M:%S or %H:%M +/- a "-" if negative
    let (positive, offset): (bool, &str) = match offset.chars().next() {
        Some('-') => (false, &offset[1..]),
        _ => (true, offset),
    };

    let offset = match offset {
        offset if NaiveTime::parse_from_str(offset, "%H:%M:%S").is_ok() => {
            NaiveTime::parse_from_str(offset, "%H:%M:%S").unwrap()
        }
        offset if NaiveTime::parse_from_str(offset, "%H:%M").is_ok() => {
            NaiveTime::parse_from_str(offset, "%H:%M").unwrap()
        }
        _ => panic!("Error parsing offset! Expected the format to be one of: %H:%M:%S | %H:%M"),
    };

    let offset = offset.signed_duration_since(NaiveTime::from_hms(0, 0, 0));

    if positive {
        offset
    } else {
        -offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;
    #[test]
    fn test_parse_date() {
        let expected = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        // standard usage, just passing in a date
        let result = parse_date(Some("2020-03-25"), "%Y-%m-%d", None);
        assert_eq!(expected, result);

        // but if you want to use a snazzy format, that is ok, too
        let result = parse_date(Some("25 March 2020"), "%d %B %Y", None);
        assert_eq!(expected, result);

        // and so is providing a custom timezone
        let expected = expected
            .with_timezone(&FixedOffset::east(3600))
            .with_hour(12)
            .unwrap();
        let result = parse_date(Some("25 Mar 2020"), "%d %b %Y", Some("+01:00"));
        assert_eq!(expected, result);

        // if no user arguments are passed in, then return the Local date
        let expected = Local::today()
            .and_hms(12, 0, 0)
            .with_timezone(&FixedOffset::from_offset(Local::now().offset()));
        let result = parse_date(None, "%Y-%m%-d", None);
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_parse_date_wrong_format_fails() {
        let _result = parse_date(Some("2020-03-25"), "%Y-%m-%Y", None);
    }

    #[test]
    #[should_panic]
    fn test_parse_date_wrong_tz_fails() {
        let _result = parse_date(Some("2020-03-25"), "%Y-%m-%d", Some("00:00"));
    }

    #[test]
    fn test_parse_event() {
        let params = [
            (Event::Sunrise, "sunrise"),
            (Event::Sunrise, "sunRISE"),
            (Event::Sunrise, "  sunrisE"),
            (Event::Sunset, "sunset"),
            (Event::Sunset, "sunSET  "),
        ];

        for (expected, arg) in params.iter() {
            assert_eq!(*expected, parse_event(*arg));
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_event_fails() {
        let _event = parse_event("sun rise");
    }
}
