extern crate corelocation_rs;

use super::errors::*;
use super::Result;
use corelocation_rs::Locator;
use serde::Deserialize;

pub struct Location {
    pub lat: f64,
    pub long: f64,
}

impl From<(f64, f64)> for Location {
    fn from(loc: (f64, f64)) -> Self {
        match loc {
            (lat, long) => Location {
                lat: lat,
                long: long,
            },
        }
    }
}

impl From<corelocation_rs::Location> for Location {
    fn from(loc: corelocation_rs::Location) -> Self {
        Location {
            lat: loc.latitude,
            long: loc.longitude,
        }
    }
}

impl From<FreeGeoApiLocation> for Location {
    fn from(loc: FreeGeoApiLocation) -> Self {
        Location {
            lat: loc.latitude,
            long: loc.longitude,
        }
    }
}

pub fn location_from_string(location: &str) -> Result<Location> {
    match &location[..1] {
        "@" => location_from_coords(&location[1..]),
        "#" => location_from_name(&location[1..]),
        "!" => location_from_auto(&location[1..]),
        "." => location_from_network(),
        _ => Err(SunshineError::MalformedLocationString),
    }
}

fn location_from_auto(fallback: &str) -> Result<Location> {
    location_from_corelocation()
        .or_else(|_| location_from_network())
        .or_else(|_| location_from_string(fallback))
}

fn location_from_coords(coords: &str) -> Result<Location> {
    let coords: Vec<&str> = coords.split(' ').collect();

    let lat = match coords.get(0) {
        Some(val) => Some(val.parse()),
        None => None,
    };

    let long = match coords.get(1) {
        Some(val) => Some(val.parse()),
        None => None,
    };

    match (lat, long) {
        // there has got to be a prettier way of doing this
        (Some(lat), Some(long)) => match (lat, long) {
            (Ok(lat), Ok(long)) => Ok(Location::from((lat, long))),
            _ => Err(SunshineError::MalformedLocationString),
        },
        _ => Err(SunshineError::MalformedLocationString),
    }
}

fn location_from_name(_name: &str) -> Result<Location> {
    panic!("unimplemented")
}

#[derive(Deserialize)]
struct FreeGeoApiLocation {
    latitude: f64,
    longitude: f64,
}

fn location_from_network() -> Result<Location> {
    let api_url = "https://freegeoip.app/json/";
    let resp = reqwest::blocking::get(api_url)?;
    let body = resp.text()?;
    let location: FreeGeoApiLocation = serde_json::from_str(&body[..])?;

    Ok(Location::from(location))
}

#[cfg(target_os = "macos")]
fn location_from_corelocation() -> Result<Location> {
    match corelocation_rs::Location::get() {
        Ok(location) => Ok(Location::from(location)),
        Err(cause) => Err(SunshineError::CoreLocationError(cause)),
    }
}

#[cfg(not(target_os = "macos"))]
fn location_from_corelocation() -> Result<Location> {
    Err(SunshineError::CoreLocationUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_good_location_from_coords() {
        assert_eq!(location_from_coords("49.9 11.5").unwrap().lat, 49.9);
        assert_eq!(location_from_coords("49.9 11.5").unwrap().long, 11.5);
    }

    #[test]
    fn test_malformed_location_from_coords() {
        assert!(location_from_coords("foobar").is_err())
    }
}
