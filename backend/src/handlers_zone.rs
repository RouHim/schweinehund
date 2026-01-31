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
struct ApiZone {
    id: String,
    name: String,
    emoji: String,
    weekday: i64,
    color: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateZoneRequest {
    name: Option<String>,
    emoji: Option<String>,
    weekday: Option<i64>,
    color: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateZoneRequest {
    name: Option<String>,
    emoji: Option<String>,
    weekday: Option<i64>,
    color: Option<String>,
}

pub async fn get_zones(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let rows = sqlx::query(
        "SELECT id, name, emoji, weekday, color \
         FROM zones \
         ORDER BY weekday ASC, created_at ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    let items: Vec<ApiZone> = rows
        .into_iter()
        .map(|row| ApiZone {
            id: row.get("id"),
            name: row.get("name"),
            emoji: row.get("emoji"),
            weekday: row.get("weekday"),
            color: row.get("color"),
        })
        .collect();

    Ok(warp::reply::json(&Items { items }))
}

pub async fn create_zone(
    req: CreateZoneRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let name = req.name.unwrap_or_default();
    let emoji = req.emoji.unwrap_or_default();
    let color = req.color.unwrap_or_default();

    if name.trim().is_empty() || color.trim().is_empty() || req.weekday.is_none() {
        return Err(error::bad_request("name, weekday, and color are required"));
    }

    let weekday = req.weekday.unwrap_or(0);
    let id = uuid::Uuid::new_v4().simple().to_string();
    let now = db::now_rfc3339();

    sqlx::query(
        "INSERT INTO zones (id, name, emoji, weekday, color, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name.trim())
    .bind(emoji.trim())
    .bind(weekday)
    .bind(color.trim())
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    let api = ApiZone {
        id,
        name: name.trim().to_string(),
        emoji: emoji.trim().to_string(),
        weekday,
        color: color.trim().to_string(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&api),
        StatusCode::OK,
    ))
}

pub async fn update_zone(
    id: String,
    req: UpdateZoneRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let existing = sqlx::query("SELECT id FROM zones WHERE id = ?")
        .bind(&id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    if existing.is_none() {
        return Err(error::not_found("zone not found"));
    }

    let row = sqlx::query("SELECT name, emoji, weekday, color FROM zones WHERE id = ?")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let mut name: String = row.get("name");
    let mut emoji: String = row.get("emoji");
    let mut weekday: i64 = row.get("weekday");
    let mut color: String = row.get("color");

    if let Some(v) = req.name {
        name = v;
    }
    if let Some(v) = req.emoji {
        emoji = v;
    }
    if let Some(v) = req.weekday {
        weekday = v;
    }
    if let Some(v) = req.color {
        color = v;
    }

    if name.trim().is_empty() || color.trim().is_empty() {
        return Err(error::bad_request("name, weekday, and color are required"));
    }

    let now = db::now_rfc3339();
    sqlx::query(
        "UPDATE zones SET name = ?, emoji = ?, weekday = ?, color = ?, updated_at = ? WHERE id = ?",
    )
    .bind(name.trim())
    .bind(emoji.trim())
    .bind(weekday)
    .bind(color.trim())
    .bind(&now)
    .bind(&id)
    .execute(&mut *tx)
    .await
    .map_err(|e| error::internal(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let api = ApiZone {
        id,
        name: name.trim().to_string(),
        emoji: emoji.trim().to_string(),
        weekday,
        color: color.trim().to_string(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&api),
        StatusCode::OK,
    ))
}

pub async fn delete_zone(id: String, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    let exists = sqlx::query("SELECT 1 FROM zones WHERE id = ?")
        .bind(&id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    if exists.is_none() {
        return Err(error::not_found("zone not found"));
    }

    let now = db::now_rfc3339();
    sqlx::query("UPDATE tasks SET zone_id = NULL, updated_at = ? WHERE zone_id = ?")
        .bind(&now)
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    sqlx::query("DELETE FROM zones WHERE id = ?")
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| error::internal(e.to_string()))?;

    Ok(warp::reply::with_status(
        warp::reply(),
        StatusCode::NO_CONTENT,
    ))
}
