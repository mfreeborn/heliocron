use std::result;

use super::{enums::Event, errors::HeliocronError};

type Result<T> = result::Result<T, HeliocronError>;

pub fn parse_event(event: &str, custom_altitude: Option<f64>) -> Result<Event> {
    Event::new(event, custom_altitude)
}

#[cfg(test)]
mod tests {
    use super::*;

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
