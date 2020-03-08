use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    pub fn from_decimal_degrees(latitude: &str, longitude: &str) -> Coordinates {
        Coordinates {
            latitude: Coordinates::parse_coordinate(latitude),
            longitude: Coordinates::parse_coordinate(longitude),
        }
    }

    fn parse_coordinate(coordinate: &str) -> f64 {
        let compass_direction: &str = &coordinate[coordinate.len() - 1..].to_lowercase();

        let compass_correction = match compass_direction {
            "n" | "e" => 1.0,
            "w" | "s" => -1.0,
            _ => panic!("Expected latitude/longitude to end with one of: N, S, E, W"),
        };

        let parsed_coordinate: f64 = coordinate[..coordinate.len() - 1]
            .parse()
            .expect("Error, expected a float!");

        parsed_coordinate * compass_correction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_coordinates() {
        let params = [
            (50.0, "50.0N"),
            (50.0, "-50.0S"),
            (-33.9, "33.9S"),
            (18.552, "18.552E"),
            (-26.02, "26.020W"),
            (-26.02, "-26.020E"),
        ];

        for (expected, arg) in params.iter() {
            assert_eq!(*expected, Coordinates::parse_coordinate(*arg))
        }
    }
}
