use chrono::Datelike;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::convert::Infallible;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

use crate::{db, notifications};

#[derive(Debug)]
struct DatabaseError;
impl reject::Reject for DatabaseError {}

#[derive(Debug)]
struct NotFoundError;
impl reject::Reject for NotFoundError {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    message: String,
}

#[derive(Deserialize)]
struct SettingsUpdate {
    notification_enabled: bool,
    notification_time: String,
}

#[derive(Serialize)]
struct SettingsResponse {
    notification_enabled: bool,
    notification_time: String,
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    name: String,
    description: Option<String>,
    zone: Option<String>,
    day_of_week: i64,
}

#[derive(Deserialize)]
struct UpdateTaskRequest {
    name: String,
    description: Option<String>,
    zone: Option<String>,
    day_of_week: i64,
}

#[derive(Deserialize)]
struct CreateDeepCleaningRequest {
    name: String,
    description: Option<String>,
    zone: Option<String>,
}

#[derive(Deserialize)]
struct UpdateDeepCleaningRequest {
    name: String,
    description: Option<String>,
    zone: Option<String>,
}

#[derive(Deserialize)]
struct ReorderRequest {
    order: Vec<i64>,
}

fn with_db(pool: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn api_routes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    health()
        .or(get_today_tasks(pool.clone()))
        .or(toggle_task(pool.clone()))
        .or(get_deep_cleaning(pool.clone()))
        .or(complete_deep_task(pool.clone()))
        .or(create_deep_cleaning(pool.clone()))
        .or(update_deep_cleaning(pool.clone()))
        .or(delete_deep_cleaning(pool.clone()))
        .or(reorder_deep_cleaning(pool.clone()))
        .or(get_settings(pool.clone()))
        .or(update_settings(pool.clone()))
        .or(create_task(pool.clone()))
        .or(update_task(pool.clone()))
        .or(delete_task(pool.clone()))
        .or(debug_reset(pool.clone()))
        .or(debug_notify(pool))
        .with(cors)
}

fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})))
}

fn get_today_tasks(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / "today")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_db(pool))
        .and_then(handle_get_today_tasks)
}

async fn handle_get_today_tasks(
    params: std::collections::HashMap<String, String>,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let day_of_week = if let Some(day) = params.get("day_of_week") {
        day.parse::<i64>()
            .unwrap_or_else(|_| chrono::Local::now().weekday().num_days_from_monday() as i64 + 1)
    } else {
        chrono::Local::now().weekday().num_days_from_monday() as i64 + 1
    };

    let tasks = db::get_today_tasks(&pool, day_of_week)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&tasks))
}

fn toggle_task(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / i64 / "toggle")
        .and(warp::post())
        .and(with_db(pool))
        .and_then(handle_toggle_task)
}

async fn handle_toggle_task(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::toggle_task(&pool, id).await.map_err(|e| {
        if e.to_string().contains("no rows") {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    let tasks = db::get_today_tasks(
        &pool,
        chrono::Local::now().weekday().num_days_from_monday() as i64 + 1,
    )
    .await
    .map_err(|_| reject::custom(DatabaseError))?;

    let task = tasks
        .into_iter()
        .find(|t| t.id == id)
        .ok_or_else(|| reject::custom(NotFoundError))?;

    Ok(warp::reply::json(&task))
}

fn get_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning")
        .and(warp::get())
        .and(with_db(pool))
        .and_then(handle_get_deep_cleaning)
}

async fn handle_get_deep_cleaning(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let tasks = db::get_deep_cleaning_queue(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&tasks))
}

fn complete_deep_task(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / i64 / "complete")
        .and(warp::post())
        .and(with_db(pool))
        .and_then(handle_complete_deep_task)
}

async fn handle_complete_deep_task(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::complete_deep_task(&pool, id).await.map_err(|e| {
        if e.to_string().contains("no rows") {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::json(&SuccessResponse {
        message: "Deep cleaning task completed and moved to end of queue".to_string(),
    }))
}

#[allow(dead_code)]
fn create_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_create_deep_cleaning)
}

#[allow(dead_code)]
async fn handle_create_deep_cleaning(
    req: CreateDeepCleaningRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if req.name.is_empty() || req.name.len() > 255 {
        return Err(reject::custom(DatabaseError));
    }

    let task = db::create_deep_cleaning_task(
        &pool,
        &req.name,
        req.description.as_deref(),
        req.zone.as_deref(),
    )
    .await
    .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::with_status(
        warp::reply::json(&task),
        StatusCode::CREATED,
    ))
}

#[allow(dead_code)]
fn update_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / i64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_update_deep_cleaning)
}

#[allow(dead_code)]
async fn handle_update_deep_cleaning(
    id: i64,
    req: UpdateDeepCleaningRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if req.name.is_empty() || req.name.len() > 255 {
        return Err(reject::custom(DatabaseError));
    }

    let task = db::update_deep_cleaning_task(
        &pool,
        id,
        &req.name,
        req.description.as_deref(),
        req.zone.as_deref(),
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("no rows") {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::json(&task))
}

#[allow(dead_code)]
fn delete_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / i64)
        .and(warp::delete())
        .and(with_db(pool))
        .and_then(handle_delete_deep_cleaning)
}

#[allow(dead_code)]
async fn handle_delete_deep_cleaning(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::delete_deep_cleaning_task(&pool, id).await.map_err(|e| {
        if e.to_string().contains("no rows") {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            message: "Deep cleaning task deleted successfully".to_string(),
        }),
        StatusCode::NO_CONTENT,
    ))
}

#[allow(dead_code)]
fn reorder_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / "reorder")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_reorder_deep_cleaning)
}

#[allow(dead_code)]
async fn handle_reorder_deep_cleaning(
    req: ReorderRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let tasks = db::reorder_deep_cleaning_queue(&pool, &req.order)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&tasks))
}

fn get_settings(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "settings")
        .and(warp::get())
        .and(with_db(pool))
        .and_then(handle_get_settings)
}

async fn handle_get_settings(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let settings = db::get_app_settings(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SettingsResponse {
        notification_enabled: settings.notification_enabled,
        notification_time: settings.notification_time,
    }))
}

fn update_settings(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "settings")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_update_settings)
}

async fn handle_update_settings(
    update: SettingsUpdate,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let settings = db::AppSettings {
        notification_enabled: update.notification_enabled,
        notification_time: update.notification_time,
    };

    db::update_app_settings(&pool, &settings)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SettingsResponse {
        notification_enabled: settings.notification_enabled,
        notification_time: settings.notification_time,
    }))
}

fn create_task(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_create_task)
}

async fn handle_create_task(
    req: CreateTaskRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if req.name.is_empty() || req.name.len() > 255 {
        return Err(reject::custom(DatabaseError));
    }

    if req.day_of_week != -1 && (req.day_of_week < 1 || req.day_of_week > 7) {
        return Err(reject::custom(DatabaseError));
    }

    let task = db::create_daily_task(
        &pool,
        &req.name,
        req.description.as_deref(),
        req.zone.as_deref(),
        req.day_of_week,
    )
    .await
    .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::with_status(
        warp::reply::json(&task),
        StatusCode::CREATED,
    ))
}

fn update_task(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / i64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_update_task)
}

async fn handle_update_task(
    id: i64,
    req: UpdateTaskRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if req.name.is_empty() || req.name.len() > 255 {
        return Err(reject::custom(DatabaseError));
    }

    if req.day_of_week != -1 && (req.day_of_week < 1 || req.day_of_week > 7) {
        return Err(reject::custom(DatabaseError));
    }

    let task = db::update_daily_task(
        &pool,
        id,
        &req.name,
        req.description.as_deref(),
        req.zone.as_deref(),
        req.day_of_week,
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("no rows") {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::json(&task))
}

fn delete_task(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / i64)
        .and(warp::delete())
        .and(with_db(pool))
        .and_then(handle_delete_task)
}

async fn handle_delete_task(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::delete_daily_task(&pool, id).await.map_err(|e| {
        if e.to_string().contains("no rows") {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            message: "Task deleted successfully".to_string(),
        }),
        StatusCode::NO_CONTENT,
    ))
}

fn debug_reset(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "reset")
        .and(warp::post())
        .and(with_db(pool))
        .and_then(handle_debug_reset)
}

async fn handle_debug_reset(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    crate::scheduler::trigger_reset_now(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SuccessResponse {
        message: "Daily tasks reset successfully".to_string(),
    }))
}

fn debug_notify(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "notify")
        .and(warp::post())
        .and(with_db(pool))
        .and_then(handle_debug_notify)
}

async fn handle_debug_notify(_pool: SqlitePool) -> Result<impl Reply, Rejection> {
    notifications::send_test_notification()
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SuccessResponse {
        message: "Test notification sent to schweinehund".to_string(),
    }))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Route not found".to_string(),
            }),
            StatusCode::NOT_FOUND,
        ))
    } else if err.find::<NotFoundError>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Resource not found".to_string(),
            }),
            StatusCode::NOT_FOUND,
        ))
    } else if err.find::<DatabaseError>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Database error occurred".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if err.find::<warp::reject::InvalidQuery>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Invalid query parameters".to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ))
    } else if err.find::<warp::body::BodyDeserializeError>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Invalid request body".to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Internal server error".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
