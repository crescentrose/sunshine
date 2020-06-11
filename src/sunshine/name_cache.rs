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

static CACHE_FILENAME: &str = "location_cache.json";

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
    /// Load cache from a file
    fn load() -> Result<LocationCache, SunshineError>;
    /// Save the current cache file
    fn save(&self) -> Result<(), SunshineError>;
    /// Retrieve an item from the cache
    fn get(&self, location_name: &str) -> Option<Location>;
    /// Store an item to the cache
    fn set(&mut self, location_name: &str, location: &Location);
    /// Attempt to retrieve an item from the cache. If it doesn't exist, call the `on_miss` closure
    /// and then store and save its return value.
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
        self.cache_data.data.get(location_name).copied()
    }

    fn set(&mut self, location_name: &str, location: &Location) {
        self.cache_data
            .data
            .insert(String::from(location_name), *location);
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
        None => return Err(SunshineError::CacheDirectoryUnavailable),
    };

    std::fs::create_dir_all(project_dirs.cache_dir())
        .map_err(|_| SunshineError::CacheDirectoryUnavailable)?;

    Ok(project_dirs.cache_dir().join(CACHE_FILENAME))
}

fn deserialize_json(filename: &PathBuf) -> Result<CacheData, SunshineError> {
    let loaded_data = fs::read_to_string(filename).map_err(|_| SunshineError::CacheLoadError)?;

    serde_json::from_str(&loaded_data[..]).map_err(SunshineError::CacheDeserializationError)
}

fn serialize_and_save(cache: &LocationCache) -> Result<(), SunshineError> {
    let filename = cache.filename();
    File::create(filename)
        .map_err(|_| SunshineError::CacheWriteError)?
        .write_all(
            serde_json::to_string(&cache.cache_data)
                .map_err(SunshineError::CacheSerializationError)?
                .as_bytes(),
        )
        .map_err(|_| SunshineError::CacheWriteError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LOCATION: Location = Location {
        lat: 48.2082,
        long: 16.3738,
    };

    #[test]
    fn cache_get_set() {
        let cache = setup_test_cache();

        assert_eq!(cache.get("Vienna").unwrap(), TEST_LOCATION);
    }

    #[test]
    fn cache_fetch_when_exists() {
        let mut cache = setup_test_cache();
        assert_eq!(
            cache
                .fetch("Vienna", || { panic!("should not execute!") })
                .unwrap(),
            TEST_LOCATION
        );
    }

    #[test]
    fn cache_fetch_when_does_not_exist() {
        let mut cache = setup_test_cache();
        let alexandria = Location {
            lat: 31.2001,
            long: 29.9187,
        };
        assert_eq!(
            cache.fetch("Alexandria", || { Ok(alexandria) }).unwrap(),
            alexandria
        );
    }

    #[test]
    fn cache_fetch_when_errors_out() {
        let mut cache = setup_test_cache();
        assert!(cache
            .fetch("Basiltown", || {
                Err(SunshineError::MalformedLocationString)
            })
            .is_err())
    }

    fn setup_test_cache() -> LocationCache {
        let mut cache = LocationCache::new().unwrap();
        cache.set("Vienna", &TEST_LOCATION);
        cache
    }
}
