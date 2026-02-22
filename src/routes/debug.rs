use sqlx::SqlitePool;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

use crate::{db, notifications, scheduler};

use super::errors::DatabaseError;
use super::types::SuccessResponse;

pub(super) fn routes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    debug_reset(pool.clone())
        .or(debug_reset_all(pool.clone()))
        .or(debug_trigger_notification(pool.clone()))
        .or(debug_notify_status(pool.clone()))
        .or(debug_notify(pool))
}

fn debug_reset(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "reset")
        .and(warp::post())
        .and(super::with_db(pool))
        .and_then(handle_debug_reset)
}

async fn handle_debug_reset(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    scheduler::trigger_reset_now(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            message: "Daily tasks reset successfully".to_string(),
        }),
        StatusCode::OK,
    ))
}

fn debug_reset_all(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "reset-all")
        .and(warp::post())
        .and(super::with_db(pool))
        .and_then(handle_debug_reset_all)
}

async fn handle_debug_reset_all(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::reset_all_data(&pool).await.map_err(|e| {
        tracing::error!("reset_all_data failed: {e}");
        reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            message: "All data reset to initial seed state successfully".to_string(),
        }),
        StatusCode::OK,
    ))
}

fn debug_trigger_notification(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "trigger-notification")
        .and(warp::post())
        .and(super::with_db(pool))
        .and_then(handle_trigger_notification)
}

async fn handle_trigger_notification(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    notifications::send_daily_reminder(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            message: "notification_triggered".to_string(),
        }),
        StatusCode::OK,
    ))
}

fn debug_notify_status(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "notify-status")
        .and(warp::get())
        .and(super::with_db(pool))
        .and_then(handle_debug_notify_status)
}

async fn handle_debug_notify_status(_pool: SqlitePool) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status(
        warp::reply::json(&notifications::get_runtime_config()),
        StatusCode::OK,
    ))
}

fn debug_notify(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "debug" / "notify")
        .and(warp::post())
        .and(super::with_db(pool))
        .and_then(handle_debug_notify)
}

async fn handle_debug_notify(_pool: SqlitePool) -> Result<impl Reply, Rejection> {
    notifications::send_test_notification()
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            message: "Test notification sent to schweinehund".to_string(),
        }),
        StatusCode::OK,
    ))
}
