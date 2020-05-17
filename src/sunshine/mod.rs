mod errors;

extern crate sunrise;
extern crate chrono;
extern crate corelocation_rs;
extern crate structopt;

use structopt::StructOpt;
use chrono::prelude::*;
use corelocation_rs::{Location, Locator};
use errors::SunshineError;

type Result<T> = std::result::Result<T, SunshineError>;

pub enum TimeOfDay {
    Day,
    Night
}

pub struct Measurements {
    pub sunrise: DateTime<FixedOffset>,
    pub sunset: DateTime<FixedOffset>,
    pub time_of_day: TimeOfDay
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sunshine")]
pub struct Opt {
    /// Location string to calculate sunrise and sunset for
    /// Format as "@lat long", e.g. "@45.815 15.9819"
    pub location: String,

    /// Output current time of day ("day" or "night") instead of accurate sunset time
    #[structopt(short, long)]
    pub simple: bool,

    /// Format string, based on chrono's strftime format
    #[structopt(short, long, default_value = "%c")]
    pub format: String
}

pub fn calculate(opt: Opt) -> Result<Measurements> {
    let now: DateTime<Local> = Local::now();
    let offset = now.offset();
    let (lat, long) = location_from_string(&opt.location[..])?;
    let (sunrise_ts, sunset_ts) = sunrise::sunrise_sunset(
            lat,
            long,
            now.date().year(),
            now.date().month(),
            now.date().day());

    let sunrise = offset.from_utc_datetime(
        &NaiveDateTime::from_timestamp(sunrise_ts, 0)
    );

    let sunset = offset.from_utc_datetime(
        &NaiveDateTime::from_timestamp(sunset_ts, 0)
    );

    let time_of_day = match now.timestamp() {
        d if d > sunrise_ts && d < sunset_ts => TimeOfDay::Day,
        _ => TimeOfDay::Night
    };

    // The compiler will probably inline this anyway
    Ok(Measurements {
        sunrise: sunrise,
        sunset: sunset,
        time_of_day: time_of_day
    })
}

fn location_from_string(location: &str) -> Result<(f64, f64)> {
    match &location[..1] {
        "@" => location_from_coords(&location[1..]),
        "#" => location_from_name(&location[1..]),
        "!" => location_from_auto(&location[1..]),
        _ => Err(SunshineError::MalformedLocationString)
    }
}

fn location_from_coords(coords: &str) -> Result<(f64, f64)> {
    let coords: Vec<&str> = coords.split(' ').collect();

    let lat = match coords.get(0) {
        Some(val) => Some(val.parse()),
        None => None
    };

    let lng = match coords.get(1) {
        Some(val) => Some(val.parse()),
        None => None
    };

    match(lat, lng) {
        (Some(lat), Some(lng)) => match(lat, lng) {
            (Ok(lat), Ok(lng)) => Ok((lat, lng)),
            _ => Err(SunshineError::MalformedLocationString)
        },
        _ => Err(SunshineError::MalformedLocationString)
    }
}

fn location_from_name(_name: &str) -> Result<(f64, f64)> {
    panic!("unimplemented")
}

// Attempt to read location from the system using CoreLocation
// If that fails, fall back to a default value
fn location_from_auto(fallback: &str) -> Result<(f64, f64)> {
    match Location::get() {
        Ok(location) => Ok((location.latitude, location.longitude)),
        Err(_) => location_from_string(fallback)
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
