#[cfg(target_os = "macos")]
extern crate corelocation_rs;
#[cfg(target_os = "macos")]
use corelocation_rs::Locator;

use super::errors::*;
use super::name_cache::LocationCache;
use super::name_cache::LocationCacher;
use super::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct Location {
    pub lat: f64,
    pub long: f64,
}

impl From<(f64, f64)> for Location {
    fn from(loc: (f64, f64)) -> Self {
        Location {
            lat: loc.0,
            long: loc.1,
        }
    }
}

#[cfg(target_os = "macos")]
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

impl TryFrom<NominatimLocation> for Location {
    type Error = SunshineError;

    fn try_from(loc: NominatimLocation) -> Result<Self> {
        location_from_coords(&format!("{} {}", loc.lat, loc.lon)[..])
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

#[derive(Deserialize, Clone)]
pub struct NominatimLocation {
    lat: String,
    lon: String,
}

fn location_from_name(name: &str) -> Result<Location> {
    let mut cache = LocationCache::load().or_else(|_| LocationCache::new())?;

    cache.fetch(name, || {
        #[cfg(not(test))]
        let api_url = "https://nominatim.openstreetmap.org";
        #[cfg(test)]
        let api_url = &mockito::server_url();

        let client = reqwest::blocking::Client::new();
        // We are fairly certain our hardcoded URLs won't cause a panic.
        let url = Url::parse(api_url).unwrap().join("search").unwrap();
        let request = client
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                "sunshine/0.2.0 (https://github.com/crescentrose/sunshine)",
            )
            .query(&[("q", name), ("format", "json")])
            .build()?;

        let resp = client.execute(request)?;
        let body = resp.text()?;
        let locations: Vec<NominatimLocation> = serde_json::from_str(&body[..])?;

        match locations.first() {
            Some(location) => Location::try_from(location.clone()),
            None => Err(SunshineError::UnknownLocationName),
        }
    })
}

#[derive(Deserialize)]
struct FreeGeoApiLocation {
    latitude: f64,
    longitude: f64,
}

fn location_from_network() -> Result<Location> {
    #[cfg(not(test))]
    let api_url = "https://freegeoip.app";
    #[cfg(test)]
    let api_url = &mockito::server_url();

    let resp = reqwest::blocking::get(&format!("{}/json/", api_url))?;
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
use mockito;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_known_good_location_from_coords() {
        assert_eq!(location_from_coords("49.9 11.5").unwrap().lat, 49.9);
        assert_eq!(location_from_coords("49.9 11.5").unwrap().long, 11.5);
    }

    #[test]
    fn test_malformed_location_from_coords() {
        assert!(location_from_coords("foobar").is_err())
    }

    #[test]
    fn test_from_network() {
        let mock = mock("GET", "/json/")
            .with_status(200)
            .with_body(
                r#"{
                "ip":"0.0.0.0",
                "country_code":"HR", "country_name":"Croatia",
                "region_code":"21", "region_name":"City of Zagreb", "city":"Zagreb",
                "zip_code":"10000", "time_zone":"Europe/Zagreb",
                "latitude":45.8293, "longitude":15.9793, "metro_code":0}"#,
            )
            .create();
        let location = location_from_network().unwrap();

        mock.assert();

        assert_eq!(location.lat, 45.8293);
        assert_eq!(location.long, 15.9793);
    }

    #[test]
    fn test_from_name() {
        let mock = mock("GET", "/search?q=Amsterdam&format=json")
            .with_status(200)
            .with_body(
                r#"[{
                "lat":"52.37454030000001", "lon":"4.897975505617977",
                "display_name":"Amsterdam, North Holland, Netherlands, The Netherlands"},
                {"lat":"52.3727598", "lon":"4.8936041",
                "display_name":"Amsterdam, North Holland, Netherlands, The Netherlands"}]"#,
            )
            .create();

        let location = location_from_name(&"Amsterdam").unwrap();

        mock.assert();

        assert_eq!(location.lat, 52.37454030000001);
        assert_eq!(location.long, 4.897975505617977);
    }
}
