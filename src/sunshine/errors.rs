extern crate corelocation_rs;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)] // We know CoreLocationUnavailable won't be used on non-Mac systems
pub enum SunshineError {
    MalformedLocationString,
    CoreLocationUnavailable,
    UnknownLocationName,
    CoreLocationError(corelocation_rs::Error),
    ApiError(reqwest::Error),
    JsonError(serde_json::Error),
    CacheSerializationError(serde_json::Error),
    CacheDeserializationError(serde_json::Error),
    CacheLoadError,
    CacheWriteError,
    CacheDirectoryUnavailable,
}

impl fmt::Display for SunshineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SunshineError::MalformedLocationString => write!(f, "malformed location string"),
            SunshineError::CoreLocationError(_) => write!(f, "corelocation failure"),
            SunshineError::CoreLocationUnavailable => write!(f, "corelocation unavailable"),
            SunshineError::UnknownLocationName => write!(f, "requested location can not be found"),
            SunshineError::ApiError(err) => write!(f, "api connection error: {:?}", err),
            SunshineError::JsonError(err) => write!(f, "api deserialization error: {:?}", err),
            SunshineError::CacheSerializationError(err) => {
                write!(f, "cache serialization error: {:?}", err)
            }
            SunshineError::CacheDeserializationError(err) => {
                write!(f, "cache serialization error: {:?}", err)
            }
            SunshineError::CacheDirectoryUnavailable => {
                write!(f, "system cache directory could not be accessed")
            }
            SunshineError::CacheLoadError => write!(f, "cache could not be loaded"),
            SunshineError::CacheWriteError => write!(f, "could not write to cache"),
        }
    }
}

impl Error for SunshineError {
    fn cause(&self) -> Option<&dyn Error> {
        match &*self {
            SunshineError::ApiError(cause) => Some(cause),
            SunshineError::CoreLocationError(cause) => Some(cause),
            SunshineError::JsonError(cause) => Some(cause),
            SunshineError::CacheSerializationError(cause) => Some(cause),
            SunshineError::CacheDeserializationError(cause) => Some(cause),
            _ => None,
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
