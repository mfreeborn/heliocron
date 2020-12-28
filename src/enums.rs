use std::result;

use super::errors::{ConfigErrorKind, HeliocronError};

type Result<T> = result::Result<T, HeliocronError>;

#[derive(Debug, PartialEq, Clone)]
pub enum TimeOfDay {
    AM,
    PM,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Sunrise {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    Sunset {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    CivilDawn {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    CivilDusk {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    NauticalDawn {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    NauticalDusk {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    AstronomicalDawn {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    AstronomicalDusk {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    CustomAM {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    CustomPM {
        degrees_below_horizon: f64,
        time_of_day: TimeOfDay,
    },
    SolarNoon,
}

impl Event {
    pub fn new(event: &str, custom_altitude: Option<f64>) -> Result<Event> {
        let event = event.trim().to_lowercase();
        match event.as_str() {
            "sunrise" => Ok(Event::Sunrise {
                degrees_below_horizon: 0.833,
                time_of_day: TimeOfDay::AM,
            }),
            "sunset" => Ok(Event::Sunset {
                degrees_below_horizon: 0.833,
                time_of_day: TimeOfDay::PM,
            }),
            "civil_dawn" => Ok(Event::CivilDawn {
                degrees_below_horizon: 6.0,
                time_of_day: TimeOfDay::AM,
            }),
            "civil_dusk" => Ok(Event::CivilDusk {
                degrees_below_horizon: 6.0,
                time_of_day: TimeOfDay::PM,
            }),
            "nautical_dawn" => Ok(Event::NauticalDawn {
                degrees_below_horizon: 12.0,
                time_of_day: TimeOfDay::AM,
            }),
            "nautical_dusk" => Ok(Event::NauticalDusk {
                degrees_below_horizon: 12.0,
                time_of_day: TimeOfDay::PM,
            }),
            "astronomical_dawn" => Ok(Event::AstronomicalDawn {
                degrees_below_horizon: 18.0,
                time_of_day: TimeOfDay::AM,
            }),
            "astronomical_dusk" => Ok(Event::AstronomicalDusk {
                degrees_below_horizon: 18.0,
                time_of_day: TimeOfDay::PM,
            }),
            // we can unwrap here safely because Structopt already checks that a
            // custom_altitude is supplied when custom_{am|pm} is the chosen event
            "custom_am" => Ok(Event::CustomAM {
                degrees_below_horizon: custom_altitude.unwrap(),
                time_of_day: TimeOfDay::AM,
            }),
            "custom_pm" => Ok(Event::CustomPM {
                degrees_below_horizon: custom_altitude.unwrap(),
                time_of_day: TimeOfDay::PM,
            }),
            "solar_noon" => Ok(Event::SolarNoon),
            _ => Err(HeliocronError::Config(ConfigErrorKind::InvalidEvent)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_custom_event_instantiation() {
        let sunrise = Event::new("sunrise", None).unwrap();
        let expected_deg = 0.833;
        let expected_tod = TimeOfDay::AM;
        if let Event::Sunrise {
            degrees_below_horizon,
            time_of_day,
        } = sunrise
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let sunset = Event::new("sunset", None).unwrap();
        let expected_deg = 0.833;
        let expected_tod = TimeOfDay::PM;
        if let Event::Sunset {
            degrees_below_horizon,
            time_of_day,
        } = sunset
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let civil_dawn = Event::new("civil_dawn", None).unwrap();
        let expected_deg = 6.0;
        let expected_tod = TimeOfDay::AM;
        if let Event::CivilDawn {
            degrees_below_horizon,
            time_of_day,
        } = civil_dawn
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let civil_dusk = Event::new("civil_dusk", None).unwrap();
        let expected_deg = 6.0;
        let expected_tod = TimeOfDay::PM;
        if let Event::CivilDusk {
            degrees_below_horizon,
            time_of_day,
        } = civil_dusk
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let nautical_dawn = Event::new("nautical_dawn", None).unwrap();
        let expected_deg = 12.0;
        let expected_tod = TimeOfDay::AM;
        if let Event::NauticalDawn {
            degrees_below_horizon,
            time_of_day,
        } = nautical_dawn
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let nautical_dusk = Event::new("nautical_dusk", None).unwrap();
        let expected_deg = 12.0;
        let expected_tod = TimeOfDay::PM;
        if let Event::NauticalDusk {
            degrees_below_horizon,
            time_of_day,
        } = nautical_dusk
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let astronomical_dawn = Event::new("astronomical_dawn", None).unwrap();
        let expected_deg = 18.0;
        let expected_tod = TimeOfDay::AM;
        if let Event::AstronomicalDawn {
            degrees_below_horizon,
            time_of_day,
        } = astronomical_dawn
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let astronomical_dusk = Event::new("astronomical_dusk", None).unwrap();
        let expected_deg = 18.0;
        let expected_tod = TimeOfDay::PM;
        if let Event::AstronomicalDusk {
            degrees_below_horizon,
            time_of_day,
        } = astronomical_dusk
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let _solar_noon = Event::new("solar_noon", None).unwrap();

        let made_up_event = Event::new("made_up_event", None);
        assert!(made_up_event.is_err());
    }

    #[test]
    fn test_custom_event_instantiation() {
        let custom_am = Event::new("custom_am", Some(8.5)).unwrap();
        let expected_deg = 8.5;
        let expected_tod = TimeOfDay::AM;
        if let Event::CustomAM {
            degrees_below_horizon,
            time_of_day,
        } = custom_am
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };

        let custom_pm = Event::new("custom_pm", Some(-8.5)).unwrap();
        let expected_deg = -8.5;
        let expected_tod = TimeOfDay::PM;
        if let Event::CustomPM {
            degrees_below_horizon,
            time_of_day,
        } = custom_pm
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };
    }

    #[test]
    fn test_non_custom_ignores_altitude() {
        let sunrise = Event::new("sunrise", Some(8.5)).unwrap();
        let expected_deg = 0.833;
        let expected_tod = TimeOfDay::AM;
        if let Event::Sunrise {
            degrees_below_horizon,
            time_of_day,
        } = sunrise
        {
            assert_eq!(expected_deg, degrees_below_horizon);
            assert_eq!(expected_tod, time_of_day);
        } else {
            unreachable!()
        };
    }
}
