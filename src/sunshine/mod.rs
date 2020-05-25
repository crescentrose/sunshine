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
    let now: DateTime<Local> = Local::now();
    let offset = now.offset();
    let location = location_from_string(&opt.location[..])?;
    let (sunrise_ts, sunset_ts) = sunrise::sunrise_sunset(
        location.lat,
        location.long,
        now.date().year(),
        now.date().month(),
        now.date().day(),
    );

    let sunrise = offset.from_utc_datetime(&NaiveDateTime::from_timestamp(sunrise_ts, 0));

    let sunset = offset.from_utc_datetime(&NaiveDateTime::from_timestamp(sunset_ts, 0));

    let time_of_day = match now.timestamp() {
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
