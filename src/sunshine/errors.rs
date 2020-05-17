extern crate corelocation_rs;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum SunshineError {
    MalformedLocationString,
    CoreLocationError(corelocation_rs::Error)
}

impl fmt::Display for SunshineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SunshineError::MalformedLocationString => write!(f, "malformed location string"),
            SunshineError::CoreLocationError(_) => write!(f, "corelocation failure")
        }
    }
}

impl Error for SunshineError {
    fn description(&self) -> &str {
        match *self {
            SunshineError::MalformedLocationString => "malformed location string",
            SunshineError::CoreLocationError(_) => "corelocation failure"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match &*self {
            SunshineError::CoreLocationError(cause) => Some(cause),
            SunshineError::MalformedLocationString => None
        }
    }
}

