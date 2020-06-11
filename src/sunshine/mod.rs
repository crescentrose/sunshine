mod errors;
mod locators;
mod name_cache;

extern crate chrono;
extern crate structopt;
extern crate sunrise;

use chrono::prelude::*;
use errors::SunshineError;
use locators::location_from_string;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, SunshineError>;

#[derive(Debug, PartialEq)]
pub enum TimeOfDay {
    Day,
    Night,
}

pub struct Measurements {
    pub sunrise: DateTime<FixedOffset>,
    pub sunset: DateTime<FixedOffset>,
    pub time_of_day: TimeOfDay,
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
    pub format: String,
}

pub fn calculate(opt: Opt) -> Result<Measurements> {
    let now = Local::now();
    let now: DateTime<FixedOffset> =
        DateTime::<FixedOffset>::from(Local::now()).with_timezone(now.offset());

    calculate_for_time(now, opt.location)
}

pub fn calculate_for_time(time: DateTime<FixedOffset>, location: String) -> Result<Measurements> {
    let location = location_from_string(&location[..])?;
    let (sunrise_ts, sunset_ts) = sunrise::sunrise_sunset(
        location.lat,
        location.long,
        time.date().year(),
        time.date().month(),
        time.date().day(),
    );

    let offset = time.offset();
    let sunrise = offset.from_utc_datetime(&NaiveDateTime::from_timestamp(sunrise_ts, 0));
    let sunset = offset.from_utc_datetime(&NaiveDateTime::from_timestamp(sunset_ts, 0));

    let time_of_day = match time.timestamp() {
        d if d > sunrise_ts && d < sunset_ts => TimeOfDay::Day,
        _ => TimeOfDay::Night,
    };

    // The compiler will probably inline this anyway
    Ok(Measurements {
        sunrise,
        sunset,
        time_of_day,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_location_string() {
        let location = String::from("@45.81 15.98");
        let time = DateTime::parse_from_rfc3339("2020-06-11T16:20:00+02:00").unwrap();
        let calculated = calculate_for_time(time, location).unwrap();

        assert_eq!(format!("{}", calculated.sunrise.format("%H:%M")), "05:05");
        assert_eq!(format!("{}", calculated.sunset.format("%H:%M")), "20:45");
        assert_eq!(calculated.time_of_day, TimeOfDay::Day);
    }
}
