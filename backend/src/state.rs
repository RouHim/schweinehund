use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::{Task, Zone};
use crate::storage::{Result as StorageResult, Storage};

#[derive(Debug)]
pub struct AppState {
    pub tasks: RwLock<Vec<Task>>,
    pub zones: RwLock<Vec<Zone>>,
    pub storage: Storage,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(data_dir: &str) -> StorageResult<Self> {
        let storage = Storage::new(data_dir)?;
        let tasks = storage.load_tasks()?;
        let zones = storage.load_zones()?;

        Ok(Self {
            tasks: RwLock::new(tasks),
            zones: RwLock::new(zones),
            storage,
        })
    }
}
