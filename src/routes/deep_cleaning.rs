use serde::Deserialize;
use sqlx::SqlitePool;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

use crate::db;

use super::errors::{is_not_found_anyhow, DatabaseError, NotFoundError};
use super::types::SuccessResponse;
use super::validation::is_valid_name;

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

pub(super) fn routes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_deep_cleaning(pool.clone())
        .or(complete_deep_task(pool.clone()))
        .or(create_deep_cleaning(pool.clone()))
        .or(update_deep_cleaning(pool.clone()))
        .or(delete_deep_cleaning(pool.clone()))
        .or(reorder_deep_cleaning(pool))
}

fn get_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning")
        .and(warp::get())
        .and(super::with_db(pool))
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
        .and(super::with_db(pool))
        .and_then(handle_complete_deep_task)
}

async fn handle_complete_deep_task(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::complete_deep_task(&pool, id).await.map_err(|e| {
        if is_not_found_anyhow(&e) {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::json(&SuccessResponse {
        message: "Deep cleaning task completed and moved to end of queue".to_string(),
    }))
}

fn create_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning")
        .and(warp::post())
        .and(warp::body::json())
        .and(super::with_db(pool))
        .and_then(handle_create_deep_cleaning)
}

async fn handle_create_deep_cleaning(
    req: CreateDeepCleaningRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if !is_valid_name(&req.name) {
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

fn update_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / i64)
        .and(warp::put())
        .and(warp::body::json())
        .and(super::with_db(pool))
        .and_then(handle_update_deep_cleaning)
}

async fn handle_update_deep_cleaning(
    id: i64,
    req: UpdateDeepCleaningRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    if !is_valid_name(&req.name) {
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
        if is_not_found_anyhow(&e) {
            reject::custom(NotFoundError)
        } else {
            reject::custom(DatabaseError)
        }
    })?;

    Ok(warp::reply::json(&task))
}

fn delete_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / i64)
        .and(warp::delete())
        .and(super::with_db(pool))
        .and_then(handle_delete_deep_cleaning)
}

async fn handle_delete_deep_cleaning(id: i64, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    db::delete_deep_cleaning_task(&pool, id)
        .await
        .map_err(|e| {
            if is_not_found_anyhow(&e) {
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

fn reorder_deep_cleaning(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "deep-cleaning" / "reorder")
        .and(warp::post())
        .and(warp::body::json())
        .and(super::with_db(pool))
        .and_then(handle_reorder_deep_cleaning)
}

async fn handle_reorder_deep_cleaning(
    req: ReorderRequest,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let tasks = db::reorder_deep_cleaning_queue(&pool, &req.order)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&tasks))
}
