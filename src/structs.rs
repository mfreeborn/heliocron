use std::{fmt, result};

use chrono::{DateTime, FixedOffset, NaiveTime};
use serde::Deserialize;

use super::errors::{ConfigErrorKind, HeliocronError};

type Result<T> = result::Result<T, HeliocronError>;

#[derive(Debug)]
pub struct EventTime {
    pub datetime: Option<DateTime<FixedOffset>>,
}

impl EventTime {
    pub fn new(datetime: Option<DateTime<FixedOffset>>) -> EventTime {
        EventTime { datetime }
    }

    pub fn is_some(&self) -> bool {
        if self.datetime.is_some() {
            true
        } else {
            false
        }
    }

    pub fn time(&self) -> Option<NaiveTime> {
        match self.datetime {
            Some(datetime) => Some(datetime.time()),
            None => None,
        }
    }
}

impl fmt::Display for EventTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.datetime {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            }
        )
    }
}

impl From<Option<DateTime<FixedOffset>>> for EventTime {
    fn from(datetime: Option<DateTime<FixedOffset>>) -> EventTime {
        EventTime::new(datetime)
    }
}

fn invalid_coordinates_error(msg: &'static str) -> HeliocronError {
    HeliocronError::Config(ConfigErrorKind::InvalidCoordindates(msg))
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Latitude {
    pub value: f64,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Longitude {
    pub value: f64,
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compass_direction = if self.value.is_sign_positive() {
            "N"
        } else {
            "S"
        };
        write!(f, "Latitude: {:.4}{}", self.value.abs(), compass_direction)
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compass_direction = if self.value.is_sign_positive() {
            "E"
        } else {
            "W"
        };
        write!(f, "Longitude: {:.4}{}", self.value.abs(), compass_direction)
    }
}

pub trait Coordinate: Sized {
    fn from_decimal_degrees(coordinate: &str) -> Result<Self>;

    fn to_radians(&self) -> f64;

    fn parse_compass_direction(coordinate: &str) -> Result<char>;
    fn parse_decimal_degrees(coordinate: &str) -> Result<f64>;
    fn compass_correction(compass_direction: char) -> Result<f64>;
}

impl Coordinate for Latitude {
    fn from_decimal_degrees(latitude: &str) -> Result<Latitude> {
        // strictly, the latitude format must be a positive float or integer with an upper or lowercase 'N' or 'S'

        let compass_direction: char = Self::parse_compass_direction(latitude)?;
        let decimal_degrees: f64 = Self::parse_decimal_degrees(latitude)?;
        let compass_correction: f64 = Self::compass_correction(compass_direction)?;

        Ok(Latitude {
            value: decimal_degrees * compass_correction,
        })
    }

    fn parse_compass_direction(latitude: &str) -> Result<char> {
        match latitude
            .to_lowercase()
            .chars()
            .last()
            .ok_or(HeliocronError::Config(
                ConfigErrorKind::InvalidCoordindates("No coordinates found"),
            ))? {
            c if c == 'n' || c == 's' => Ok(c),
            _ => Err(invalid_coordinates_error(
                "Latitude must end with 'N' or 'S'",
            )),
        }
    }

    fn parse_decimal_degrees(latitude: &str) -> Result<f64> {
        latitude[..latitude.len() - 1]
            .parse()
            .map_err(|_| invalid_coordinates_error("Latitude must be a positive value followed by a compass direction ('N' or 'S')"))
            .and_then(|n: f64| match n {
                n if n.is_sign_positive() => match n {
                    n if (0.0..=90.0).contains(&n) => Ok(n),
                    _ => Err(invalid_coordinates_error("Latitude must be a positive value between 0.0 and 90.0")),
                },
                _ => Err(invalid_coordinates_error("Latitude must be a positive value between 0.0 and 90.0")),
            })
    }

    fn compass_correction(compass_direction: char) -> Result<f64> {
        match compass_direction {
            'n' => Ok(1.0),
            's' => Ok(-1.0),
            _ => Err(invalid_coordinates_error(
                "Latitude must be a positive value followed by a compass direction ('N' or 'S')",
            )),
        }
    }

    fn to_radians(&self) -> f64 {
        self.value.to_radians()
    }
}

impl Coordinate for Longitude {
    fn from_decimal_degrees(longitude: &str) -> Result<Self> {
        // strictly, the longitude format must be a positive float or integer with an upper or lowercase 'W' or 'E'

        let compass_direction: char = Self::parse_compass_direction(longitude)?;
        let decimal_degrees: f64 = Self::parse_decimal_degrees(longitude)?;
        let compass_correction: f64 = Self::compass_correction(compass_direction)?;

        Ok(Longitude {
            value: decimal_degrees * compass_correction,
        })
    }

    fn parse_compass_direction(longitude: &str) -> Result<char> {
        match longitude
            .to_lowercase()
            .chars()
            .last()
            .ok_or(HeliocronError::Config(
                ConfigErrorKind::InvalidCoordindates("No coordinates found"),
            ))? {
            c if c == 'w' || c == 'e' => Ok(c),
            _ => Err(invalid_coordinates_error(
                "Longitude must end with 'W' or 'E'.",
            )),
        }
    }

    fn parse_decimal_degrees(longitude: &str) -> Result<f64> {
        longitude[..longitude.len() - 1]
            .parse()
            .map_err(|_| invalid_coordinates_error("Longitude must be a positive value followed by a compass direction ('W' or 'E')"))
            .and_then(|n: f64| match n {
                n if n.is_sign_positive() => match n {
                    n if (0.0..=180.0).contains(&n) => Ok(n),
                    _ => Err(invalid_coordinates_error("Longitude must be a positive value between 0.0 and 180.0")),
                },
                _ => Err(invalid_coordinates_error("Longitude must be a positive value between 0.0 and 180.0")),
            })
    }

    fn compass_correction(compass_direction: char) -> Result<f64> {
        match compass_direction {
            'e' => Ok(1.0),
            'w' => Ok(-1.0),
            _ => Err(invalid_coordinates_error(
                "Longitude must be a positive value followed by a compass direction ('W' or 'E')",
            )),
        }
    }

    fn to_radians(&self) -> f64 {
        self.value.to_radians()
    }
}

impl Coordinates {
    pub fn from_decimal_degrees(latitude: &str, longitude: &str) -> Result<Coordinates> {
        let latitude = Latitude::from_decimal_degrees(latitude)?;
        let longitude = Longitude::from_decimal_degrees(longitude)?;
        Ok(Coordinates {
            latitude,
            longitude,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_latitude() {
        let params = [
            (50.0, "50.0N"),
            (-50.0, "50.0S"),
            (-33.9, "33.9S"),
            (18.552, "18.552n"),
            (-26.02, "26.020s"),
            (90.0, "90.0n"),
            (0.0, "0.0n"),
        ];

        for (expected, arg) in params.iter() {
            assert_eq!(
                *expected,
                Latitude::from_decimal_degrees(*arg).unwrap().value
            )
        }
    }
    #[test]
    fn test_parse_longitude() {
        let params = [
            (50.0, "50.0E"),
            (-50.0, "50.0W"),
            (-33.9, "33.9W"),
            (18.552, "18.552e"),
            (-26.02, "26.020w"),
            (180.0, "180.0e"),
            (0.0, "0.0e"),
        ];

        for (expected, arg) in params.iter() {
            assert_eq!(
                *expected,
                Longitude::from_decimal_degrees(*arg).unwrap().value
            )
        }
    }
}
