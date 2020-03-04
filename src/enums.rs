use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Event {
    Sunrise,
    Sunset,
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: &str = &s.trim().to_lowercase();

        match s {
            "sunrise" => Ok(Self::Sunrise),
            "sunset" => Ok(Self::Sunset),
            _ => Err(()),
        }
    }
}
