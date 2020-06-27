use std::result;

use super::errors::{ConfigErrorKind, HeliocronError};

type Result<T> = result::Result<T, HeliocronError>;

#[derive(Debug, PartialEq)]
pub enum Event {
    Sunrise,
    Sunset,
    CivilDawn,
    CivilDusk,
    NauticalDawn,
    NauticalDusk,
    AstronomicalDawn,
    AstronomicalDusk,
}

impl Event {
    pub fn new(event: &str) -> Result<Event> {
        let event = event.trim().to_lowercase();
        match event.as_str() {
            "sunrise" => Ok(Event::Sunrise),
            "sunset" => Ok(Event::Sunset),
            "civil_dawn" => Ok(Event::CivilDawn),
            "civil_dusk" => Ok(Event::CivilDusk),
            "nautical_dawn" => Ok(Event::NauticalDawn),
            "nautical_dusk" => Ok(Event::NauticalDusk),
            "astronomical_dawn" => Ok(Event::AstronomicalDawn),
            "astronomical_dusk" => Ok(Event::AstronomicalDusk),
            _ => Err(HeliocronError::Config(ConfigErrorKind::InvalidEvent)),
        }
    }
}

#[derive(Debug)]
pub enum TwilightType {
    Civil,
    Nautical,
    Astronomical,
}
