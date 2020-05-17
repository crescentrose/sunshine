extern crate corelocation_rs;

use corelocation_rs::Locator;
use super::errors::*;
use super::Result;

pub struct Location {
    pub lat: f64,
    pub long: f64
}

impl From<(f64, f64)> for Location {
    fn from(loc: (f64, f64)) -> Self {
        match loc {
            (lat, long) => Location { lat: lat, long: long }
        }
    }
}

impl From<corelocation_rs::Location> for Location {
    fn from(loc: corelocation_rs::Location) -> Self {
        Location { lat: loc.latitude, long: loc.longitude }
    }
}

pub fn location_from_string(location: &str) -> Result<Location> {
    match &location[..1] {
        "@" => location_from_coords(&location[1..]),
        "#" => location_from_name(&location[1..]),
        "!" => location_from_auto(&location[1..]),
        _ => Err(SunshineError::MalformedLocationString)
    }
}

// Attempt to read location from the system using macOS CoreLocation
// If that fails, try to infer location from timezone data.
// If all else fails, fall back to a default value
fn location_from_auto(fallback: &str) -> Result<Location> {
    location_from_corelocation().or_else(|_| {
        location_from_string(fallback)
    })
}

fn location_from_coords(coords: &str) -> Result<Location> {
    let coords: Vec<&str> = coords.split(' ').collect();

    let lat = match coords.get(0) {
        Some(val) => Some(val.parse()),
        None => None
    };

    let long = match coords.get(1) {
        Some(val) => Some(val.parse()),
        None => None
    };

    match(lat, long) {
        // there has got to be a prettier way of doing this
        (Some(lat), Some(long)) => match(lat, long) {
            (Ok(lat), Ok(long)) => Ok(Location::from((lat, long))),
            _ => Err(SunshineError::MalformedLocationString)
        },
        _ => Err(SunshineError::MalformedLocationString)
    }
}

fn location_from_name(_name: &str) -> Result<Location> {
    panic!("unimplemented")
}

fn location_from_corelocation() -> Result<Location> {
    match corelocation_rs::Location::get() {
        Ok(location) => Ok(Location::from(location)),
        Err(cause) => Err(SunshineError::CoreLocationError(cause))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_good_location_from_coords() {
        assert_eq!(location_from_coords("49.9 11.5").unwrap(), (49.9, 11.5))
    }

    #[test]
    fn test_malformed_location_from_coords() {
        assert!(location_from_coords("foobar").is_err())
    }
}
