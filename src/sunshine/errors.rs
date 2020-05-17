use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum SunshineError {
    MalformedLocationString
}

impl fmt::Display for SunshineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SunshineError::MalformedLocationString => write!(f, "malformed location string")
        }
    }
}

impl Error for SunshineError {
    fn description(&self) -> &str {
        match *self {
            SunshineError::MalformedLocationString => "malformed location string"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SunshineError::MalformedLocationString => None
        }
    }
}

