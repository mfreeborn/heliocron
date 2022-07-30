use std::{fmt, ops::Deref, result};

use chrono::{DateTime, FixedOffset, NaiveTime};
use serde::{Deserialize, Serialize};

use super::errors::{ConfigErrorKind, HeliocronError};

type Result<T, E = HeliocronError> = result::Result<T, E>;

/// A newtype representing an optional datetime. This allows us to provide custom
/// serialization methods when converting to a String or JSON.
#[derive(Debug)]
pub struct EventTime(pub Option<DateTime<FixedOffset>>);

impl EventTime {
    pub fn new(datetime: Option<DateTime<FixedOffset>>) -> Self {
        Self(datetime)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn time(&self) -> Option<NaiveTime> {
        self.0.map(|dt| dt.time())
    }
}

impl Serialize for EventTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(datetime) => serializer.serialize_str(&datetime.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }
}

impl fmt::Display for EventTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            }
        )
    }
}

fn invalid_coordinates_error(msg: &'static str) -> HeliocronError {
    HeliocronError::Config(ConfigErrorKind::InvalidCoordindates(msg))
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Latitude(pub f64);

impl Deref for Latitude {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compass_direction = if self.is_sign_positive() { "N" } else { "S" };
        write!(f, "{:.4}{}", self.abs(), compass_direction)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Longitude(pub f64);

impl Deref for Longitude {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compass_direction = if self.is_sign_positive() { "E" } else { "W" };
        write!(f, "{:.4}{}", self.abs(), compass_direction)
    }
}

pub trait Coordinate: Sized {
    fn from_decimal_degrees(coordinate: &str) -> Result<Self>;

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

        Ok(Latitude(decimal_degrees * compass_correction))
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
}

impl Coordinate for Longitude {
    fn from_decimal_degrees(longitude: &str) -> Result<Self> {
        // strictly, the longitude format must be a positive float or integer with an upper or lowercase 'W' or 'E'

        let compass_direction: char = Self::parse_compass_direction(longitude)?;
        let decimal_degrees: f64 = Self::parse_decimal_degrees(longitude)?;
        let compass_correction: f64 = Self::compass_correction(compass_direction)?;

        Ok(Longitude(decimal_degrees * compass_correction))
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
            assert_eq!(*expected, Latitude::from_decimal_degrees(*arg).unwrap().0)
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
            assert_eq!(*expected, Longitude::from_decimal_degrees(*arg).unwrap().0)
        }
    }

    #[test]
    fn test_serialize_event_time() {
        let dt = DateTime::parse_from_rfc3339("2022-06-11T12:00:00+01:00").unwrap();
        let et = EventTime::new(Some(dt));
        // serialize to rfc3339
        let expected = serde_json::json!("2022-06-11T12:00:00+01:00");
        assert_eq!(serde_json::to_value(et).unwrap(), expected);

        let et = EventTime::new(None);
        //serialize to null
        let expected = serde_json::json!(null);
        assert_eq!(serde_json::to_value(et).unwrap(), expected);
    }

    #[test]
    fn test_display_event_time() {
        let dt = DateTime::parse_from_rfc3339("2022-06-11T12:00:00+01:00").unwrap();
        let et = EventTime::new(Some(dt));
        let expected = "2022-06-11 12:00:00 +01:00";
        assert_eq!(et.to_string(), expected);

        let et = EventTime::new(None);
        let expected = "Never";
        assert_eq!(et.to_string(), expected);
    }

    #[test]
    fn test_serialize_coordinates() {
        // coordinates are serialized to decimal degree notation
        let coord = Coordinates {
            latitude: Latitude(51.1),
            longitude: Longitude(3.56),
        };
        let expected = serde_json::json!({"latitude": 51.1, "longitude": 3.56});
        assert_eq!(serde_json::to_value(coord).unwrap(), expected);
    }

    #[test]
    fn test_display_latitude() {
        let lat = Latitude(51.1);
        let expected = "51.1000N";
        assert_eq!(lat.to_string(), expected);

        let lat = Latitude(-51.1);
        let expected = "51.1000S";
        assert_eq!(lat.to_string(), expected);
    }

    #[test]
    fn test_display_longitude() {
        let lon = Longitude(51.1);
        let expected = "51.1000E";
        assert_eq!(lon.to_string(), expected);

        let lon = Longitude(-51.1);
        let expected = "51.1000W";
        assert_eq!(lon.to_string(), expected);
    }
}
