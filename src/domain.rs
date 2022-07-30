use std::{fmt::Display, ops::RangeInclusive};

const LATITUDE_RANGE: RangeInclusive<f64> = RangeInclusive::new(-90.0, 90.0);
const LONGITUDE_RANGE: RangeInclusive<f64> = RangeInclusive::new(-180.0, 180.0);

/// Represents a latitude in decimal degrees. Valid values are from -90.0..=+90.0.
/// Positive values are to the north, whilst negative values are to the south.
#[derive(PartialEq, Debug, Clone, serde::Serialize)]
pub struct Latitude(f64);

impl Latitude {
    /// Create a new instance of `Latitude` from an f64.
    pub fn new(value: f64) -> Result<Self, String> {
        match LATITUDE_RANGE.contains(&value) {
            true => Ok(Self(value)),
            false => Err(format!(
                "Latitude must be between -90.0 and 90.0, inclusive. Found `{value}`."
            )),
        }
    }

    /// Create a new instance of `Latitude` from an &str, such as when parsing command line
    /// arguments.
    pub fn parse(value: &str) -> Result<Self, String> {
        value
            .parse()
            .map_err(|_| {
                format!("Latitude must be between -90.0 and 90.0, inclusive. Found `{value}`.")
            })
            .and_then(Self::new)
    }
}

impl Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Latitude {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a longitude in decimal degrees. Valid values are from -180.0..=+180.0.
/// Positive values are to the east, whilst negative values are to the west.
#[derive(PartialEq, Debug, Clone, serde::Serialize)]
pub struct Longitude(f64);

impl Longitude {
    /// Create a new `Longitude` from an f64.
    pub fn new(value: f64) -> Result<Self, String> {
        match LONGITUDE_RANGE.contains(&value) {
            true => Ok(Self(value)),
            false => Err(format!(
                "Longitude must be between -180.0 and 180.0, inclusive. Found '{value}'."
            )),
        }
    }

    /// Create a new instance of `Longitude` from an &str, such as when parsing command line
    /// arguments.
    pub fn parse(value: &str) -> Result<Self, String> {
        value
            .parse()
            .map_err(|_| {
                format!("Longitude must be between -180.0 and 180.0, inclusive. Found `{value}`.")
            })
            .and_then(Self::new)
    }
}

impl Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Longitude {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents poisition on a map described by a latitude and longitude.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl Coordinates {
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_latitude() {
        let lat = Latitude::new(15.1234).unwrap();
        let expected = Latitude(15.1234);

        assert_eq!(lat, expected);
    }

    #[test]
    fn test_new_latitude_with_valid_values() {
        let vals = &[-90.0, -89.9999999999, -0.0, 0.0, 90.0];

        for val in vals {
            assert!(Latitude::new(*val).is_ok());
        }
    }

    #[test]
    fn test_new_latitude_with_invalid_values() {
        let vals = &[-180.0, -90.00000000001, 90.00000001, 100.0];

        for val in vals {
            assert!(Latitude::new(*val).is_err());
        }
    }

    #[test]
    fn test_parse_latitude_with_valid_values() {
        let vals = &["-90.0", "-89.9999999999", "-0.0", "0.0", "90.0"];

        for val in vals {
            assert!(Latitude::parse(*val).is_ok());
        }
    }

    #[test]
    fn test_parse_latitude_with_invalid_values() {
        let vals = &["-180.0", "-90.00000000001", "90.00000001", "100.0"];

        for val in vals {
            assert!(Latitude::parse(*val).is_err());
        }
    }

    #[test]
    fn test_new_longitude() {
        let lat = Longitude::new(-150.1234).unwrap();
        let expected = Longitude(-150.1234);

        assert_eq!(lat, expected);
    }

    #[test]
    fn test_new_longitude_with_valid_values() {
        let vals = &[-180.0, -90.0, -89.9999, -0.0, 0.0, 90.0, 180.0];

        for val in vals {
            assert!(Longitude::new(*val).is_ok());
        }
    }

    #[test]
    fn test_new_longitude_with_invalid_values() {
        let vals = &[-180.1, 180.01];

        for val in vals {
            assert!(Longitude::new(*val).is_err());
        }
    }

    #[test]
    fn test_parse_longitude_with_valid_values() {
        let vals = &[
            "-180.0", "-90.0", "-89.9999", "-0.0", "0.0", "90.0", "180.0",
        ];

        for val in vals {
            assert!(Longitude::parse(*val).is_ok());
        }
    }

    #[test]
    fn test_parse_longitude_with_invalid_values() {
        let vals = &["-180.1", "180.01"];

        for val in vals {
            assert!(Longitude::parse(*val).is_err());
        }
    }

    #[test]
    fn test_new_coordinates() {
        let latitude = Latitude::new(10.0).unwrap();
        let longitude = Longitude::new(20.0).unwrap();

        let coords = Coordinates::new(latitude.clone(), longitude.clone());
        let expected = Coordinates {
            latitude,
            longitude,
        };

        assert_eq!(coords, expected);
    }
}
