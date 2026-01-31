use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::db;
use crate::error;

#[derive(Serialize)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
struct ApiTask {
    id: String,
    name: String,
    emoji: String,
    is_daily: bool,
    zone: Option<String>,
    sort_order: i64,
    completed: bool,
    completed_at: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateTaskRequest {
    name: Option<String>,
    emoji: Option<String>,
    is_daily: Option<bool>,
    zone: Option<Option<String>>,
    sort_order: Option<i64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateTaskRequest {
    name: Option<String>,
    emoji: Option<String>,
    is_daily: Option<bool>,
    zone: Option<Option<String>>,
    sort_order: Option<i64>,
    completed: Option<bool>,
    completed_at: Option<String>,
}

fn normalize_completed_at(input: &str) -> Result<Option<String>, Rejection> {
    if input.trim().is_empty() {
        return Ok(None);
    }

    let ts = DateTime::parse_from_rfc3339(input)
        .map_err(|_| error::bad_request("invalid completed_at"))?
        .with_timezone(&Utc);

    Ok(Some(ts.to_rfc3339_opts(SecondsFormat::Secs, true)))
}

fn normalize_zone(input: Option<Option<String>>) -> Option<String> {
    match input {
        Some(Some(v)) => {
            let v = v.trim().to_string();
            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        }
        Some(None) => None,
        None => None,
    }
}

pub async fn get_tasks(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let rows = sqlx::query(
        "SELECT id, name, emoji, is_daily, zone_id, sort_order, completed, COALESCE(completed_at, '') as completed_at \
         FROM tasks \
         ORDER BY sort_order ASC, created_at ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    let items: Vec<ApiTask> = rows
        .into_iter()
        .map(|row| ApiTask {
            id: row.get("id"),
            name: row.get("name"),
            emoji: row.get("emoji"),
            is_daily: row.get::<i64, _>("is_daily") != 0,
            zone: row.get::<Option<String>, _>("zone_id"),
            sort_order: row.get("sort_order"),
            completed: row.get::<i64, _>("completed") != 0,
            completed_at: row.get("completed_at"),
        })
        .collect();

    Ok(warp::reply::json(&Items { items }))
}

pub async fn create_task(
    req: CreateTaskRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let name = req.name.unwrap_or_default();
    let emoji = req.emoji.unwrap_or_default();

    if name.trim().is_empty() || emoji.trim().is_empty() {
        return Err(error::bad_request("name and emoji are required"));
    }

    let id = uuid::Uuid::new_v4().simple().to_string();
    let now = db::now_rfc3339();
    let is_daily = req.is_daily.unwrap_or(false);
    let zone_id = normalize_zone(req.zone);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let sort_order = match req.sort_order {
        Some(v) => v,
        None => {
            let max: Option<i64> = sqlx::query_scalar("SELECT MAX(sort_order) FROM tasks")
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| error::internal(e.to_string()))?;
            max.unwrap_or(-1) + 1
        }
    };

    sqlx::query(
        "INSERT INTO tasks \
        (id, name, emoji, is_daily, sort_order, completed, completed_at, zone_id, created_at, updated_at) \
        VALUES (?, ?, ?, ?, ?, 0, NULL, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name.trim())
    .bind(emoji.trim())
    .bind(if is_daily { 1i64 } else { 0i64 })
    .bind(sort_order)
    .bind(zone_id.as_deref())
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let api = ApiTask {
        id,
        name: name.trim().to_string(),
        emoji: emoji.trim().to_string(),
        is_daily,
        zone: zone_id,
        sort_order,
        completed: false,
        completed_at: "".to_string(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&api),
        StatusCode::OK,
    ))
}

pub async fn update_task(
    id: String,
    req: UpdateTaskRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let existing = sqlx::query("SELECT is_daily, completed FROM tasks WHERE id = ?")
        .bind(&id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let Some(existing) = existing else {
        return Err(error::not_found("task not found"));
    };

    let old_is_daily = existing.get::<i64, _>("is_daily") != 0;
    let old_completed = existing.get::<i64, _>("completed") != 0;

    // Compute new values with fallbacks.
    let row = sqlx::query(
        "SELECT id, name, emoji, is_daily, zone_id, sort_order, completed, completed_at \
         FROM tasks WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    let mut name: String = row.get("name");
    let mut emoji: String = row.get("emoji");
    let mut is_daily: bool = row.get::<i64, _>("is_daily") != 0;
    let mut zone_id: Option<String> = row.get("zone_id");
    let mut sort_order: i64 = row.get("sort_order");
    let mut completed: bool = row.get::<i64, _>("completed") != 0;
    let mut completed_at: Option<String> = row.get("completed_at");

    if let Some(v) = req.name {
        name = v;
    }
    if let Some(v) = req.emoji {
        emoji = v;
    }
    if let Some(v) = req.is_daily {
        is_daily = v;
    }
    if let Some(v) = req.sort_order {
        sort_order = v;
    }
    if let Some(v) = req.completed {
        completed = v;
    }
    if let Some(v) = req.completed_at {
        completed_at = normalize_completed_at(&v)?;
    }
    if let Some(v) = req.zone {
        zone_id = normalize_zone(Some(v));
    }

    if name.trim().is_empty() || emoji.trim().is_empty() {
        return Err(error::bad_request("name and emoji are required"));
    }

    // Rotation on completion: non-daily tasks when completed transitions false -> true.
    if !old_is_daily && !old_completed && completed {
        let max_non_daily: Option<i64> =
            sqlx::query_scalar("SELECT MAX(sort_order) FROM tasks WHERE is_daily = 0")
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| error::internal(e.to_string()))?;
        sort_order = max_non_daily.unwrap_or(0) + 1;
    }

    let now = db::now_rfc3339();
    sqlx::query(
        "UPDATE tasks \
         SET name = ?, emoji = ?, is_daily = ?, zone_id = ?, sort_order = ?, completed = ?, completed_at = ?, updated_at = ? \
         WHERE id = ?",
    )
    .bind(name.trim())
    .bind(emoji.trim())
    .bind(if is_daily { 1i64 } else { 0i64 })
    .bind(zone_id.as_deref())
    .bind(sort_order)
    .bind(if completed { 1i64 } else { 0i64 })
    .bind(completed_at.as_deref())
    .bind(&now)
    .bind(&id)
    .execute(&mut *tx)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let api = ApiTask {
        id,
        name: name.trim().to_string(),
        emoji: emoji.trim().to_string(),
        is_daily,
        zone: zone_id,
        sort_order,
        completed,
        completed_at: completed_at.unwrap_or_default(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&api),
        StatusCode::OK,
    ))
}

pub async fn delete_task(id: String, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let res = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    if res.rows_affected() == 0 {
        return Err(error::not_found("task not found"));
    }

    Ok(warp::reply::with_status(
        warp::reply(),
        StatusCode::NO_CONTENT,
    ))
}
