use std::result;

use chrono::{Duration, NaiveTime};

use super::{
    enums::Event,
    errors::{ConfigErrorKind, HeliocronError},
};

type Result<T> = result::Result<T, HeliocronError>;

pub fn parse_event(event: &str, custom_altitude: Option<f64>) -> Result<Event> {
    Event::new(event, custom_altitude)
}

pub fn parse_offset(offset: &str) -> Result<Duration> {
    // offset should either be %H:%M:%S or %H:%M +/- a "-" if negative
    let (positive, offset): (bool, &str) = match offset.chars().next() {
        Some('-') => (false, &offset[1..]),
        _ => (true, offset),
    };

    let offset = match offset {
        offset if NaiveTime::parse_from_str(offset, "%H:%M:%S").is_ok() => {
            Ok(NaiveTime::parse_from_str(offset, "%H:%M:%S")?)
        }
        offset if NaiveTime::parse_from_str(offset, "%H:%M").is_ok() => {
            Ok(NaiveTime::parse_from_str(offset, "%H:%M")?)
        }
        _ => Err(HeliocronError::Config(ConfigErrorKind::ParseOffset)),
    }?;

    let offset = offset.signed_duration_since(NaiveTime::from_hms(0, 0, 0));

    if positive {
        Ok(offset)
    } else {
        Ok(-offset)
    }
}

pub fn parse_altitude(altitude: String) -> Result<f64> {
    let altitude: f64 = match altitude.parse() {
        Ok(altitude) => Ok(altitude),
        Err(_) => Err(HeliocronError::Config(ConfigErrorKind::ParseAltitude)),
    }?;

    if (-90.0..=90.0).contains(&altitude) {
        Ok(altitude)
    } else {
        Err(HeliocronError::Config(ConfigErrorKind::ParseAltitude))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_altitude() {
        let valid_altitudes = &["90.0", "8", "0", "-1.2", "-90.0"];

        for a in valid_altitudes.iter() {
            assert!(parse_altitude((*a).to_owned()).is_ok())
        }

        let invalid_altitudes = &["-90.1", "90.1", "not_an_altitude"];

        for a in invalid_altitudes.iter() {
            assert!(parse_altitude((*a).to_owned()).is_err())
        }
    }

    #[test]
    fn test_parse_offset() {
        let valid_offsets = &[
            ("12:00:00", Duration::hours(12)),
            ("12:00", Duration::hours(12)),
            ("-12:00:00", -Duration::hours(12)),
            ("23:59:59", Duration::seconds(86399)),
            ("23:59", Duration::seconds(86340)),
            ("00:59", Duration::minutes(59)),
            ("00:00", Duration::minutes(0)),
            ("0:00", Duration::minutes(0)),
            ("0:0", Duration::minutes(0)),
        ];

        for (input, expected) in valid_offsets.iter() {
            let offset = parse_offset(*input).unwrap();
            assert_eq!(*expected, offset);
        }

        let invalid_offsets = &["24:00:00"];

        for input in invalid_offsets.iter() {
            let offset = parse_offset(*input);
            assert!(offset.is_err());
        }
    }

    #[test]
    fn test_parse_event() {
        let params = [
            (
                Event::Sunrise {
                    degrees_below_horizon: 0.833,
                    time_of_day: crate::enums::TimeOfDay::AM,
                },
                "sunrise",
            ),
            (
                Event::Sunrise {
                    degrees_below_horizon: 0.833,
                    time_of_day: crate::enums::TimeOfDay::AM,
                },
                "sunRISE",
            ),
            (
                Event::Sunrise {
                    degrees_below_horizon: 0.833,
                    time_of_day: crate::enums::TimeOfDay::AM,
                },
                "  sunrisE",
            ),
            (
                Event::Sunset {
                    degrees_below_horizon: 0.833,
                    time_of_day: crate::enums::TimeOfDay::PM,
                },
                "sunset",
            ),
            (
                Event::Sunset {
                    degrees_below_horizon: 0.833,
                    time_of_day: crate::enums::TimeOfDay::PM,
                },
                "sunSET  ",
            ),
        ];

        for (expected, arg) in params.iter() {
            assert_eq!(*expected, parse_event(*arg, None).unwrap());
        }
    }

    #[test]
    fn test_parse_event_fails() {
        let event = parse_event("sun rise", None);
        assert!(event.is_err());
    }
}
