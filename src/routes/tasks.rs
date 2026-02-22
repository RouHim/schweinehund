use chrono::Datelike;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::collections::HashMap;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

use crate::db;

use super::errors::{is_not_found_anyhow, BadRequest, DatabaseError, NotFoundError};
use super::types::{AllTasksResponse, SuccessResponse};
use super::validation::{
    derive_start_date_for_create, final_interval_weeks, first_day_of_month_opt,
    interval_weeks_or_default, is_past_month, is_valid_day_of_week, is_valid_interval_weeks,
    is_valid_name, parse_calendar_month, validate_optional_start_date_for_update,
};

#[derive(Deserialize)]
struct CreateTaskRequest {
    name: String,
    description: Option<String>,
    zone: Option<String>,
    day_of_week: i64,
    interval_weeks: Option<i64>,
    start_date: Option<String>,
}

#[derive(Deserialize)]
struct UpdateTaskRequest {
    name: String,
    description: Option<String>,
    zone: Option<String>,
    day_of_week: i64,
    interval_weeks: Option<i64>,
    start_date: Option<String>,
}

#[derive(Deserialize)]
struct CalendarQuery {
    month: String,
}

pub(super) fn routes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_today_tasks(pool.clone())
        .or(get_all_tasks(pool.clone()))
        .or(calendar_tasks(pool.clone()))
        .or(toggle_task(pool.clone()))
        .or(create_task(pool.clone()))
        .or(update_task(pool.clone()))
        .or(delete_task(pool))
}

fn get_today_tasks(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / "today")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(super::with_db(pool))
        .and_then(handle_get_today_tasks)
}

fn get_all_tasks(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / "all")
        .and(warp::get())
        .and(super::with_db(pool))
        .and_then(handle_get_all_tasks)
}

fn calendar_tasks(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / "calendar")
        .and(warp::get())
        .and(warp::query::<CalendarQuery>())
        .and(super::with_db(pool))
        .and_then(handle_get_calendar)
}

async fn handle_get_today_tasks(
    params: HashMap<String, String>,
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

async fn handle_get_all_tasks(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let (daily_tasks, deep_cleaning_tasks) = db::get_all_daily_tasks(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&AllTasksResponse {
        daily_tasks,
        deep_cleaning_tasks,
    }))
}

async fn handle_get_calendar(
    query: CalendarQuery,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let query_date = parse_calendar_month(&query.month).map_err(|_| reject::custom(BadRequest))?;

    let now = chrono::Local::now();
    let current_date = now.date_naive();
    let current_month_first = first_day_of_month_opt(current_date.year(), current_date.month())
        .ok_or_else(|| reject::custom(DatabaseError))?;

    if is_past_month(query_date, current_month_first) {
        return Err(reject::custom(BadRequest));
    }

    let year = query_date.year();
    let month = query_date.month();

    let calendar = db::get_tasks_for_month(&pool, year, month)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&calendar))
}

fn toggle_task(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks" / i64 / "toggle")
        .and(warp::post())
        .and(super::with_db(pool))
        .and_then(handle_toggle_task)
}

async fn handle_toggle_task(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::toggle_task(&pool, id).await.map_err(|e| {
        if is_not_found_anyhow(&e) {
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

fn create_task(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "tasks")
        .and(warp::post())
        .and(warp::body::json())
        .and(super::with_db(pool))
        .and_then(handle_create_task)
}

async fn handle_create_task(
    req: CreateTaskRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if !is_valid_name(&req.name) {
        return Err(reject::custom(DatabaseError));
    }

    if !is_valid_day_of_week(req.day_of_week) {
        return Err(reject::custom(DatabaseError));
    }

    let interval_weeks = interval_weeks_or_default(req.interval_weeks, 1);

    if !is_valid_interval_weeks(interval_weeks) {
        return Err(reject::custom(DatabaseError));
    }

    let final_interval = final_interval_weeks(req.day_of_week, interval_weeks);

    let start_date = derive_start_date_for_create(req.start_date.as_deref())
        .map_err(|_| reject::custom(DatabaseError))?;

    let task = db::create_daily_task(
        &pool,
        &req.name,
        req.description.as_deref(),
        req.zone.as_deref(),
        req.day_of_week,
        final_interval,
        &start_date,
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
        .and(super::with_db(pool))
        .and_then(handle_update_task)
}

async fn handle_update_task(
    id: i64,
    req: UpdateTaskRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if !is_valid_name(&req.name) {
        return Err(reject::custom(DatabaseError));
    }

    if !is_valid_day_of_week(req.day_of_week) {
        return Err(reject::custom(DatabaseError));
    }

    let current_task = db::get_all_daily_tasks(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?
        .0
        .into_iter()
        .find(|t| t.id == id)
        .ok_or_else(|| reject::custom(NotFoundError))?;

    let interval_weeks = interval_weeks_or_default(req.interval_weeks, current_task.interval_weeks);

    if !is_valid_interval_weeks(interval_weeks) {
        return Err(reject::custom(DatabaseError));
    }

    validate_optional_start_date_for_update(req.start_date.as_deref())
        .map_err(|_| reject::custom(DatabaseError))?;

    let final_interval = final_interval_weeks(req.day_of_week, interval_weeks);

    let task = db::update_daily_task(
        &pool,
        id,
        &req.name,
        req.description.as_deref(),
        req.zone.as_deref(),
        req.day_of_week,
        final_interval,
        req.start_date.as_deref(),
    )
    .await
    .map_err(|e| {
        if is_not_found_anyhow(&e) {
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
        .and(super::with_db(pool))
        .and_then(handle_delete_task)
}

async fn handle_delete_task(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::delete_daily_task(&pool, id).await.map_err(|e| {
        if is_not_found_anyhow(&e) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_task_with_valid_interval() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Valid Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: Some(2),
            start_date: None,
        };

        let result = handle_create_task(req, pool).await;
        assert!(
            result.is_ok(),
            "Should accept interval_weeks between 1 and 52"
        );
    }

    #[tokio::test]
    async fn test_create_task_invalid_interval_zero() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Bad Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: Some(0),
            start_date: None,
        };

        let result = handle_create_task(req, pool).await;
        assert!(result.is_err(), "Should reject interval_weeks of 0");
    }

    #[tokio::test]
    async fn test_create_task_invalid_interval_negative() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Bad Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: Some(-5),
            start_date: None,
        };

        let result = handle_create_task(req, pool).await;
        assert!(result.is_err(), "Should reject negative interval_weeks");
    }

    #[tokio::test]
    async fn test_create_task_invalid_interval_too_large() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Bad Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: Some(53),
            start_date: None,
        };

        let result = handle_create_task(req, pool).await;
        assert!(result.is_err(), "Should reject interval_weeks > 52");
    }

    #[tokio::test]
    async fn test_create_mini_routine_ignores_interval() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Mini Routine".to_string(),
            description: None,
            zone: None,
            day_of_week: -1,
            interval_weeks: Some(10),
            start_date: None,
        };

        let result = handle_create_task(req, pool).await;
        assert!(
            result.is_ok(),
            "Should accept mini-routine (day_of_week=-1)"
        );

        if let Ok(reply) = result {
            let status = reply.into_response().status();
            assert_eq!(status, StatusCode::CREATED);
        }
    }

    #[tokio::test]
    async fn test_update_task_changes_interval() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let created = db::create_daily_task(&pool, "Original", None, None, 1, 1, "2026-02-09")
            .await
            .unwrap();

        let req = UpdateTaskRequest {
            name: "Updated".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: Some(3),
            start_date: None,
        };

        let result = handle_update_task(created.id, req, pool).await;
        assert!(result.is_ok(), "Should update task with new interval_weeks");
    }

    #[tokio::test]
    async fn test_create_task_with_user_start_date() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Future Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: None,
            start_date: Some("2026-03-01".to_string()),
        };

        let result = handle_create_task(req, pool.clone()).await;
        assert!(result.is_ok(), "Should accept user-provided start_date");

        let tasks = db::get_all_daily_tasks(&pool).await.unwrap().0;
        let task = tasks.into_iter().find(|t| t.name == "Future Task").unwrap();
        assert_eq!(task.start_date, Some("2026-03-01".to_string()));
    }

    #[tokio::test]
    async fn test_create_task_without_start_date_defaults_today() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Today Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: None,
            start_date: None,
        };

        let result = handle_create_task(req, pool.clone()).await;
        assert!(result.is_ok(), "Should accept None and default to today");

        let tasks = db::get_all_daily_tasks(&pool).await.unwrap().0;
        let task = tasks.into_iter().find(|t| t.name == "Today Task").unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(task.start_date, Some(today));
    }

    #[tokio::test]
    async fn test_create_task_with_invalid_start_date_rejected() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = CreateTaskRequest {
            name: "Bad Date Task".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: None,
            start_date: Some("not-a-date".to_string()),
        };

        let result = handle_create_task(req, pool).await;
        assert!(result.is_err(), "Should reject invalid date format");
    }

    #[tokio::test]
    async fn test_update_task_with_start_date() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let created = db::create_daily_task(&pool, "Original", None, None, 1, 1, "2026-02-09")
            .await
            .unwrap();

        let req = UpdateTaskRequest {
            name: "Updated".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: None,
            start_date: Some("2026-03-15".to_string()),
        };

        let result = handle_update_task(created.id, req, pool.clone()).await;
        assert!(result.is_ok(), "Should accept start_date update");

        let tasks = db::get_all_daily_tasks(&pool).await.unwrap().0;
        let task = tasks.into_iter().find(|t| t.id == created.id).unwrap();
        assert_eq!(task.start_date, Some("2026-03-15".to_string()));
    }

    #[tokio::test]
    async fn test_update_task_without_start_date_preserves() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let created = db::create_daily_task(&pool, "Original", None, None, 1, 1, "2026-02-09")
            .await
            .unwrap();

        let req = UpdateTaskRequest {
            name: "Updated Name".to_string(),
            description: None,
            zone: None,
            day_of_week: 1,
            interval_weeks: None,
            start_date: None,
        };

        let result = handle_update_task(created.id, req, pool.clone()).await;
        assert!(result.is_ok(), "Should accept update with None start_date");

        let tasks = db::get_all_daily_tasks(&pool).await.unwrap().0;
        let task = tasks.into_iter().find(|t| t.id == created.id).unwrap();
        assert_eq!(
            task.start_date,
            Some("2026-02-09".to_string()),
            "Should preserve existing start_date when None is passed"
        );
    }
}
