use super::{enums, structs};

use chrono::{DateTime, Duration, FixedOffset, NaiveTime, Offset, Timelike};

use super::traits::{DateTimeExt, NaiveTimeExt};

#[derive(Debug, Clone)]
struct SolarCalculationsRow {
    solar_declination: f64,
    solar_noon_fraction: f64,
    corrected_solar_elevation_angle: f64,
}

impl SolarCalculationsRow {
    pub fn new(
        date: DateTime<FixedOffset>,
        coordinates: structs::Coordinates,
    ) -> SolarCalculationsRow {
        let time_zone = date.offset().fix().local_minus_utc() as f64 / 3600.0;
        let julian_date: f64 = date.to_julian_date();

        let julian_century = (julian_date - 2451545.0) / 36525.0;

        let geometric_solar_mean_longitude =
            (280.46646 + julian_century * (36000.76983 + julian_century * 0.0003032)) % 360.0;

        let solar_mean_anomaly =
            357.52911 + julian_century * (35999.05029 - 0.0001537 * julian_century);

        let eccent_earth_orbit =
            0.016708634 - julian_century * (0.000042037 + 0.0000001267 * julian_century);

        let equation_of_the_center = solar_mean_anomaly.to_radians().sin()
            * (1.914602 - julian_century * (0.004817 + 0.000014 * julian_century))
            + (2.0 * solar_mean_anomaly).to_radians().sin()
                * (0.019993 - 0.000101 * julian_century)
            + (3.0 * solar_mean_anomaly).to_radians().sin() * 0.000289;

        let solar_true_longitude = geometric_solar_mean_longitude + equation_of_the_center;

        let solar_apparent_longitude = solar_true_longitude
            - 0.00569
            - 0.00478 * (125.04 - 1934.136 * julian_century).to_radians().sin();

        let mean_oblique_ecliptic = 23.0
            + (26.0
                + (21.448
                    - julian_century
                        * (46.815 + julian_century * (0.00059 - julian_century * 0.001813)))
                    / 60.0)
                / 60.0;

        let oblique_corrected = mean_oblique_ecliptic
            + 0.00256 * (125.04 - 1934.136 * julian_century).to_radians().cos();

        let solar_declination = ((oblique_corrected.to_radians().sin()
            * solar_apparent_longitude.to_radians().sin())
        .asin())
        .to_degrees();

        let var_y = (oblique_corrected / 2.0).to_radians().tan().powi(2);

        let equation_of_time = 4.0
            * (var_y * (geometric_solar_mean_longitude.to_radians() * 2.0).sin()
                - 2.0 * eccent_earth_orbit * solar_mean_anomaly.to_radians().sin()
                + 4.0
                    * eccent_earth_orbit
                    * var_y
                    * solar_mean_anomaly.to_radians().sin()
                    * (geometric_solar_mean_longitude.to_radians() * 2.0).cos()
                - 0.5 * var_y * var_y * (geometric_solar_mean_longitude.to_radians() * 4.0).sin()
                - 1.25
                    * eccent_earth_orbit
                    * eccent_earth_orbit
                    * (solar_mean_anomaly.to_radians() * 2.0).sin())
            .to_degrees();

        let solar_noon_fraction =
            (720.0 - 4.0 * *coordinates.longitude - equation_of_time + time_zone * 60.0) / 1440.0;

        let true_solar_time =
            (date.time().day_fraction() * 1440.0 + equation_of_time + 4.0 * *coordinates.longitude
                - 60.0 * time_zone)
                % 1440.0;

        let true_hour_angle = if true_solar_time / 4.0 < 0.0 {
            true_solar_time / 4.0 + 180.0
        } else {
            true_solar_time / 4.0 - 180.0
        };

        let solar_zenith_angle = (coordinates.latitude.to_radians().sin()
            * solar_declination.to_radians().sin()
            + coordinates.latitude.to_radians().cos()
                * solar_declination.to_radians().cos()
                * true_hour_angle.to_radians().cos())
        .acos()
        .to_degrees();

        let solar_elevation_angle = 90.0 - solar_zenith_angle;

        let atmospheric_refraction = (if solar_elevation_angle > 85.0 {
            0.0
        } else if solar_elevation_angle > 5.0 {
            58.1 / solar_elevation_angle.to_radians().tan()
                - 0.07 / solar_elevation_angle.to_radians().tan().powi(3)
                + 0.000086 / solar_elevation_angle.to_radians().tan().powi(5)
        } else if solar_elevation_angle > -0.575 {
            1735.0
                + solar_elevation_angle
                    * (103.4 + solar_elevation_angle * (-12.79 + solar_elevation_angle * 0.711))
        } else {
            -20.772 / solar_elevation_angle.to_radians().tan()
        } / 3600.0);

        let corrected_solar_elevation_angle = solar_elevation_angle + atmospheric_refraction;

        SolarCalculationsRow {
            solar_declination,
            solar_noon_fraction,
            corrected_solar_elevation_angle,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolarCalculations {
    // required parameters
    pub date: DateTime<FixedOffset>,
    pub coordinates: structs::Coordinates,

    // calculated fields
    midday_calculations: SolarCalculationsRow,
}

impl SolarCalculations {
    pub fn new(
        date: DateTime<FixedOffset>,
        coordinates: structs::Coordinates,
    ) -> SolarCalculations {
        // calculate the necessary values for midday
        let midday_calculations = SolarCalculationsRow::new(date, coordinates);

        SolarCalculations {
            date,
            coordinates,
            midday_calculations,
        }
    }

    pub fn get_solar_noon(&self) -> structs::EventTime {
        let solar_noon =
            self.day_fraction_to_datetime(self.midday_calculations.solar_noon_fraction);
        structs::EventTime {
            datetime: Some(solar_noon),
        }
    }

    fn day_fraction_to_datetime(&self, mut day_fraction: f64) -> DateTime<FixedOffset> {
        let mut date = self.date;

        // correct the date if the event rolls over to the next day, or happens on the previous day
        if day_fraction < 0.0 {
            date = date - Duration::days(1);
            day_fraction = day_fraction.abs();
        } else if day_fraction >= 1.0 {
            date = date + Duration::days(1);
            day_fraction -= 1.0;
        }

        let hour_fraction = day_fraction * 24.0;
        let minute_fraction = hour_fraction.fract() * 60.0;
        let second_fraction = minute_fraction.fract() * 60.0;

        let time = NaiveTime::from_hms(
            hour_fraction.trunc() as u32,
            minute_fraction.trunc() as u32,
            second_fraction.trunc() as u32,
        );

        date.with_hour(time.hour())
            .unwrap()
            .with_minute(time.minute())
            .unwrap()
            .with_second(time.second())
            .unwrap()
    }

    fn calculate_hour_angle(&self, degrees_below_horizon: f64) -> Option<f64> {
        let event_angle = degrees_below_horizon + 90.0;
        let hour_angle = (((event_angle.to_radians().cos()
            / (self.coordinates.latitude.to_radians().cos()
                * self
                    .midday_calculations
                    .solar_declination
                    .to_radians()
                    .cos()))
            - self.coordinates.latitude.to_radians().tan()
                * self
                    .midday_calculations
                    .solar_declination
                    .to_radians()
                    .tan())
        .acos())
        .to_degrees();

        match hour_angle.is_nan() {
            true => None,
            false => Some(hour_angle),
        }
    }

    pub fn calculate_event_time(&self, event: enums::Event) -> structs::EventTime {
        let (degrees_below_horizon, time_of_day) = match event {
            enums::Event::Sunrise {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::Sunset {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::CivilDawn {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::CivilDusk {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::NauticalDawn {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::NauticalDusk {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::AstronomicalDawn {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::AstronomicalDusk {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::CustomAM {
                degrees_below_horizon,
                time_of_day,
            }
            | enums::Event::CustomPM {
                degrees_below_horizon,
                time_of_day,
            } => (degrees_below_horizon, time_of_day),
            // we call this method having already matched against the above events
            _ => unreachable!(),
        };

        let hour_angle = self.calculate_hour_angle(degrees_below_horizon);

        match hour_angle {
            Some(hour_angle) => {
                let day_fraction = match time_of_day {
                    enums::TimeOfDay::AM => {
                        self.midday_calculations.solar_noon_fraction - (hour_angle / 360.0)
                    }
                    enums::TimeOfDay::PM => {
                        self.midday_calculations.solar_noon_fraction + (hour_angle / 360.0)
                    }
                };

                let event_time = self.day_fraction_to_datetime(day_fraction);

                structs::EventTime {
                    datetime: Some(event_time),
                }
            }
            None => structs::EventTime { datetime: None },
        }
    }

    pub fn calculate_day_length(&self) -> Duration {
        // we can unwrap here safely because we have manually validated these calls with the Event::new constructor
        let sunrise = self.calculate_event_time(enums::Event::new("sunrise", None).unwrap());
        let sunset = self.calculate_event_time(enums::Event::new("sunset", None).unwrap());

        if sunrise.is_some() & sunset.is_some() {
            // sunrise does actually occur, so it is just sunet - sunrise
            sunset.datetime.unwrap() - sunrise.datetime.unwrap()
        } else {
            // there is no sunrise, so we need to work out whether the sun was above the sunrise angle
            // at solar noon or not
            if self.calculate_max_solar_elevation() >= 0.833 {
                Duration::hours(24)
            } else {
                Duration::hours(0)
            }
        }
    }

    fn calculate_max_solar_elevation(&self) -> f64 {
        // need to get the corrected solar elevation angle at solar noon
        // we can unwrap solar noon safely, because solar noon always occurs for a given day
        SolarCalculationsRow::new(self.get_solar_noon().datetime.unwrap(), self.coordinates)
            .corrected_solar_elevation_angle
    }
}

#[cfg(test)]
mod tests {
    // note that the correctness of the maths in this module is tested elsewhere
    use super::*;

    #[test]
    fn test_day_fraction_to_time_underoverflow() {
        // when a location is selected which is in a different time zone, it is possible for the sunrise/sunset to
        // occur either the following or previous day. This results in a day fraction which is either negative
        // or >= 1
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("0.0N", "0.0W").unwrap();
        let report = SolarCalculations::new(date, coordinates);

        let params = [
            ("2020-03-26 12:00:00 +00:00", 1.5),
            ("2020-03-24 12:00:00 +00:00", -0.5),
        ];

        for (expected_time, arg) in params.iter() {
            let result = report.day_fraction_to_datetime(*arg);
            assert_eq!(*expected_time, result.to_string());
        }
    }

    #[test]
    fn test_day_fraction_to_time() {
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("0.0N", "0.0W").unwrap();
        let report = SolarCalculations::new(date, coordinates);
        let params = [
            ("2020-03-25 00:00:00 +00:00", 0.0),
            ("2020-03-25 12:00:00 +00:00", 0.5),
            ("2020-03-25 23:59:59 +00:00", 0.99999),
            ("2020-03-25 01:23:45 +00:00", 0.05816),
            ("2020-03-25 23:42:12 +00:00", 0.987639),
        ];

        for (expected, arg) in params.iter() {
            let result = report.day_fraction_to_datetime(*arg).to_string();
            assert_eq!(*expected, result);
        }
    }

    #[test]
    fn test_day_length() {
        // assert the correct length of day for a typical day
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("51.4769N", "0.0005W").unwrap();

        let solar_calculations = SolarCalculations::new(date, coordinates);

        let day_length = solar_calculations.calculate_day_length().num_seconds();
        let expected = 45113;

        assert_eq!(day_length, expected);
    }

    #[test]
    fn test_day_length_24_hour_night() {
        // assert the correct length of day when the sun never rises
        let date = DateTime::parse_from_rfc3339("2020-12-25T12:00:00+00:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("70.67299N", "23.67165E").unwrap();

        let solar_calculations = SolarCalculations::new(date, coordinates);

        let day_length = solar_calculations.calculate_day_length().num_seconds();
        let expected = 0;

        assert_eq!(day_length, expected);
    }

    #[test]
    fn test_day_length_24_hour_day() {
        // assert the correct length of day when the sun never rises
        let date = DateTime::parse_from_rfc3339("2020-06-25T12:00:00+00:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("70.67299N", "23.67165E").unwrap();

        let solar_calculations = SolarCalculations::new(date, coordinates);

        let day_length = solar_calculations.calculate_day_length().num_seconds();
        let expected = 86400;

        assert_eq!(day_length, expected);
    }
}
