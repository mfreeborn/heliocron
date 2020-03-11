use std::error;

use chrono;

#[derive(Debug)]
pub enum HeliocronError {
    Config(ConfigErrorKind),
}

#[derive(Debug)]
pub enum ConfigErrorKind {
    InvalidCoordindates(&'static str),
    InvalidTomlFile,
    ParseDate,
    InvalidEvent,
}

impl ConfigErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ConfigErrorKind::InvalidCoordindates(msg) => msg,
            ConfigErrorKind::InvalidTomlFile => {
                "Error parsing .toml file. Ensure that it is of the correct format."
            }
            ConfigErrorKind::ParseDate => {
                "Error parsing date. Ensure the date and timezone formats are correct."
            }
            ConfigErrorKind::InvalidEvent => {
                "Error parsing event. Expected one of {'sunrise' | 'sunset'}"
            }
        }
    }
}

impl std::fmt::Display for HeliocronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HeliocronError::Config(ref err) => write!(
                f,
                "Config error: {}",
                match err {
                    ConfigErrorKind::InvalidCoordindates(msg) =>
                        format!("Invalid coordinates - {}", msg),
                    ConfigErrorKind::InvalidTomlFile => err.as_str().to_string(),
                    ConfigErrorKind::ParseDate => err.as_str().to_string(),
                    ConfigErrorKind::InvalidEvent => err.as_str().to_string(),
                }
            ),
        }
    }
}

impl error::Error for HeliocronError {
    fn description(&self) -> &str {
        match *self {
            HeliocronError::Config(ref err) => err.as_str(),
        }
    }
}

impl From<chrono::ParseError> for HeliocronError {
    fn from(err: chrono::ParseError) -> Self {
        match err {
            _err => HeliocronError::Config(ConfigErrorKind::ParseDate),
        }
    }
}
