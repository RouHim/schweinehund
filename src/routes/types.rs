use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Serialize, Deserialize)]
pub(crate) struct AllTasksResponse {
    pub daily_tasks: Vec<db::DailyTask>,
    pub deep_cleaning_tasks: Vec<db::DeepCleaningTask>,
}

#[derive(Serialize)]
pub(crate) struct SuccessResponse {
    pub message: String,
}
