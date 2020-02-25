use chrono::{DateTime, Datelike, TimeZone};

pub trait DateTimeExt {
    fn to_julian_date(&self) -> f64;
}

impl<Tz: TimeZone> DateTimeExt for DateTime<Tz> {
    fn to_julian_date(&self) -> f64 {
        let (year, month, day): (i32, i32, i32) =
            (self.year(), self.month() as i32, self.day() as i32);

        (367 * year - 7 * (year + (month + 9) / 12) / 4 + 275 * month / 9 + day + 1721014) as f64
    }
}
