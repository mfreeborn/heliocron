use chrono::{DateTime, Datelike, NaiveTime, TimeZone, Timelike};

pub trait DateTimeExt {
    fn to_julian_date(&self) -> f64;
}

impl<Tz: TimeZone> DateTimeExt for DateTime<Tz> {
    fn to_julian_date(&self) -> f64 {
        // takes a DateTime<Tz> and returns the number of days elapsed since noon 1/1/4713 BC
        let (year, month, day): (i32, i32, i32) =
            (self.year(), self.month() as i32, self.day() as i32);

        let julian_day =
            (367 * year - 7 * (year + (month + 9) / 12) / 4 + 275 * month / 9 + day + 1721014)
                as f64;

        // convert to UTC
        let utc_datetime = self.naive_utc();

        // adjust for the epoch starting at 12:00 UTC
        let hour_part = if utc_datetime.hour() >= 12 {
            (utc_datetime.hour() - 12) as f64 / 24.0
        } else {
            (utc_datetime.hour() as f64 / 24.0) - 0.5
        };

        let time_part = hour_part
            + (utc_datetime.minute() as f64 / 1440.0)
            + (utc_datetime.second() as f64 / 86400.0);

        julian_day + time_part
    }
}

pub trait NaiveTimeExt {
    fn day_fraction(&self) -> f64;
}

impl NaiveTimeExt for NaiveTime {
    fn day_fraction(&self) -> f64 {
        self.num_seconds_from_midnight() as f64 / 86400.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_juilian_date() {
        let params = [
            ("2458924.00626", "2020-03-15T12:09:01+00:00"),
            ("2458923.92293", "2020-03-15T10:09:01+00:00"),
            ("2458924.00626", "2020-03-15T13:09:01+01:00"), // cover funny timezones, too
            ("2458923.92293", "2020-03-15T11:09:01+01:00"),
        ];

        for (expected, arg) in params.iter() {
            let date = DateTime::parse_from_rfc3339(*arg).unwrap();
            assert_eq!(*expected, format!("{:.5}", date.to_julian_date()));
        }
    }

    #[test]
    fn test_day_fraction() {
        let params = [
            (0.0, NaiveTime::from_hms(0, 0, 0)),
            (0.5, NaiveTime::from_hms(12, 0, 0)),
            (0.55, NaiveTime::from_hms(13, 12, 0)),
            (0.75, NaiveTime::from_hms(18, 0, 0)),
        ];

        for (expected, time) in params.iter() {
            assert_eq!(*expected, time.day_fraction())
        }
    }
}
