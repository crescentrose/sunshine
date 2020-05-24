extern crate directories_next;

use super::errors::SunshineError;
use super::locators::Location;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

static CACHE_FILENAME: &'static str = "location_cache.json";

#[derive(Debug)]
pub struct LocationCache {
    cache_data: CacheData,
    cache_filename: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheData {
    data: HashMap<String, Location>,
}

pub trait LocationCacher {
    /// Create a new cache.
    fn new() -> Result<LocationCache, SunshineError>;
    /// Load an existing cache from a file
    fn load() -> Result<LocationCache, SunshineError>;
    fn save(&self) -> Result<(), SunshineError>;
    fn get(&self, location_name: &str) -> Option<Location>;
    fn set(&mut self, location_name: &str, location: &Location);
    fn fetch<F>(&mut self, location_name: &str, on_miss: F) -> Result<Location, SunshineError>
    where
        F: Fn() -> Result<Location, SunshineError>;
}

impl LocationCacher for LocationCache {
    fn new() -> Result<LocationCache, SunshineError> {
        let filename = cache_file_path()?;
        Ok(LocationCache {
            cache_filename: filename,
            cache_data: CacheData {
                data: HashMap::new(),
            },
        })
    }

    fn load() -> Result<LocationCache, SunshineError> {
        let filename = cache_file_path()?;
        let data = deserialize_json(&filename)?;
        Ok(LocationCache {
            cache_filename: filename,
            cache_data: data,
        })
    }

    fn save(&self) -> Result<(), SunshineError> {
        serialize_and_save(self)
    }

    fn get(&self, location_name: &str) -> Option<Location> {
        self.cache_data.data.get(location_name).map(|l| *l)
    }

    fn set(&mut self, location_name: &str, location: &Location) {
        self.cache_data
            .data
            .insert(String::from(location_name), location.clone());
    }

    fn fetch<F>(&mut self, location_name: &str, on_miss: F) -> Result<Location, SunshineError>
    where
        F: Fn() -> Result<Location, SunshineError>,
    {
        match self.get(location_name) {
            Some(location) => Ok(location),
            None => {
                let location = on_miss()?;
                self.set(location_name, &location);
                self.save().map_err(|_| SunshineError::CacheWriteError)?;
                Ok(location)
            }
        }
    }
}

impl LocationCache {
    fn filename(&self) -> &PathBuf {
        &self.cache_filename
    }
}

fn cache_file_path() -> Result<PathBuf, SunshineError> {
    let project_dirs = match ProjectDirs::from("hr", "halcyon", "sunshine") {
        Some(path) => path,
        None => return Err(SunshineError::CacheLoadError),
    };

    std::fs::create_dir_all(project_dirs.cache_dir()).map_err(|_| SunshineError::CacheLoadError)?;

    Ok(project_dirs.cache_dir().join(CACHE_FILENAME))
}

fn deserialize_json(filename: &PathBuf) -> Result<CacheData, SunshineError> {
    let loaded_data = fs::read_to_string(filename).map_err(|_| SunshineError::CacheLoadError)?;
    serde_json::from_str(&loaded_data[..]).map_err(|_| SunshineError::CacheLoadError)
}

fn serialize_and_save(cache: &LocationCache) -> Result<(), SunshineError> {
    let filename = cache.filename();
    File::create(filename)
        .map_err(|_| SunshineError::CacheWriteError)?
        .write_all(
            serde_json::to_string(&cache.cache_data)
                .map_err(|_| SunshineError::CacheWriteError)?
                .as_bytes(),
        )
        .map_err(|_| SunshineError::CacheWriteError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
