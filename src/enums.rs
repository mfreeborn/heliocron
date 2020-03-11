use std::result;

use super::errors::{ConfigErrorKind, HeliocronError};

type Result<T> = result::Result<T, HeliocronError>;

#[derive(Debug, PartialEq)]
pub enum Event {
    Sunrise,
    Sunset,
}

impl Event {
    pub fn new(event: &str) -> Result<Event> {
        let event = event.trim().to_lowercase();
        match event.as_str() {
            "sunrise" => Ok(Event::Sunrise),
            "sunset" => Ok(Event::Sunset),
            _ => Err(HeliocronError::Config(ConfigErrorKind::InvalidEvent)),
        }
    }
}
