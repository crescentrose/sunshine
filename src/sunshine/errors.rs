extern crate corelocation_rs;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)] // We know CoreLocationUnavailable won't be used on non-Mac systems
pub enum SunshineError {
    MalformedLocationString,
    CoreLocationUnavailable,
    CoreLocationError(corelocation_rs::Error),
    ApiError(reqwest::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for SunshineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SunshineError::MalformedLocationString => write!(f, "malformed location string"),
            SunshineError::CoreLocationError(_) => write!(f, "corelocation failure"),
            SunshineError::CoreLocationUnavailable => write!(f, "corelocation unavailable"),
            SunshineError::ApiError(err) => write!(f, "api connection error: {:?}", err),
            SunshineError::JsonError(err) => write!(f, "api deserialization error: {:?}", err),
        }
    }
}

impl Error for SunshineError {
    fn description(&self) -> &str {
        match &*self {
            SunshineError::MalformedLocationString => "malformed location string",
            SunshineError::CoreLocationError(_) => "corelocation failure",
            SunshineError::CoreLocationUnavailable => "corelocation unavailable",
            SunshineError::ApiError(_) => "api connection error",
            SunshineError::JsonError(_) => "api deserialization error",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match &*self {
            SunshineError::ApiError(cause) => Some(cause),
            SunshineError::CoreLocationError(cause) => Some(cause),
            SunshineError::JsonError(cause) => Some(cause),
            SunshineError::MalformedLocationString => None,
            SunshineError::CoreLocationUnavailable => None,
        }
    }
}

impl From<reqwest::Error> for SunshineError {
    fn from(err: reqwest::Error) -> Self {
        SunshineError::ApiError(err)
    }
}

impl From<serde_json::Error> for SunshineError {
    fn from(err: serde_json::Error) -> Self {
        SunshineError::JsonError(err)
    }
}
