use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub emoji: String,
    pub is_daily: bool,
    pub sort_order: i32,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub zone_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub emoji: String,
    pub weekday: i32,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
