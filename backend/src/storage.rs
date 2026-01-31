use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{de::DeserializeOwned, Serialize};

use crate::models::{Task, Zone};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Clone, Debug)]
pub struct Storage {
    data_dir: PathBuf,
    tasks_path: PathBuf,
    zones_path: PathBuf,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        fs::create_dir_all(&data_dir)?;
        let tasks_path = data_dir.join("tasks.json");
        let zones_path = data_dir.join("zones.json");
        Ok(Self {
            data_dir,
            tasks_path,
            zones_path,
        })
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        load_json_file(&self.tasks_path)
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> Result<()> {
        save_json_file(&self.data_dir, &self.tasks_path, tasks)
    }

    pub fn load_zones(&self) -> Result<Vec<Zone>> {
        load_json_file(&self.zones_path)
    }

    pub fn save_zones(&self, zones: &[Zone]) -> Result<()> {
        save_json_file(&self.data_dir, &self.zones_path, zones)
    }
}

fn load_json_file<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>> {
    match fs::read(path) {
        Ok(bytes) => {
            if bytes.is_empty() {
                return Ok(Vec::new());
            }
            Ok(serde_json::from_slice(&bytes)?)
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(StorageError::Io(err)),
    }
}

fn save_json_file<T: Serialize>(data_dir: &Path, path: &Path, value: &[T]) -> Result<()> {
    fs::create_dir_all(data_dir)?;
    let data = serde_json::to_vec_pretty(value)?;
    atomic_write(path, &data)
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| io::Error::other("path has no parent directory"))?;
    let tmp_path = path.with_extension("tmp");
    let mut file = File::create(&tmp_path)?;
    file.write_all(data)?;
    file.sync_all()?;
    fs::rename(&tmp_path, path)?;
    let dir_file = File::open(dir)?;
    dir_file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    fn temp_storage() -> Storage {
        let dir = std::env::temp_dir().join(format!("schweinehund-test-{}", Uuid::new_v4()));
        Storage::new(dir).unwrap()
    }

    fn sample_task(id: &str) -> Task {
        Task {
            id: id.to_string(),
            name: "Dishes".to_string(),
            emoji: "D".to_string(),
            is_daily: true,
            sort_order: 0,
            completed: false,
            completed_at: None,
            zone_id: None,
            created_at: Utc.with_ymd_and_hms(2026, 1, 31, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 31, 10, 0, 0).unwrap(),
        }
    }

    fn sample_zone(id: &str) -> Zone {
        Zone {
            id: id.to_string(),
            name: "Kitchen".to_string(),
            emoji: "K".to_string(),
            weekday: 1,
            color: "#FF7F50".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 1, 31, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 31, 10, 0, 0).unwrap(),
        }
    }

    #[test]
    fn test_save_and_load_tasks() {
        let storage = temp_storage();
        let tasks = vec![sample_task("task-1"), sample_task("task-2")];

        storage.save_tasks(&tasks).unwrap();
        let loaded = storage.load_tasks().unwrap();

        assert_eq!(loaded, tasks);
    }

    #[test]
    fn test_save_and_load_zones() {
        let storage = temp_storage();
        let zones = vec![sample_zone("zone-1"), sample_zone("zone-2")];

        storage.save_zones(&zones).unwrap();
        let loaded = storage.load_zones().unwrap();

        assert_eq!(loaded, zones);
    }

    #[test]
    fn test_atomic_write_creates_file() {
        let storage = temp_storage();
        let tasks = vec![sample_task("task-1")];

        storage.save_tasks(&tasks).unwrap();

        let metadata = fs::metadata(storage.tasks_path).unwrap();
        assert!(metadata.is_file());
    }

    #[test]
    fn test_missing_file_returns_empty() {
        let storage = temp_storage();

        let tasks = storage.load_tasks().unwrap();
        let zones = storage.load_zones().unwrap();

        assert!(tasks.is_empty());
        assert!(zones.is_empty());
    }
}
