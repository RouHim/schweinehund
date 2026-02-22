use anyhow::Error as AnyhowError;
use serde::Serialize;
use warp::{http::StatusCode, reject, Rejection, Reply};

#[derive(Debug)]
pub(crate) struct DatabaseError;
impl reject::Reject for DatabaseError {}

#[derive(Debug)]
pub(crate) struct NotFoundError;
impl reject::Reject for NotFoundError {}

#[derive(Debug)]
pub(crate) struct BadRequest;
impl reject::Reject for BadRequest {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub(crate) fn is_not_found_anyhow(err: &AnyhowError) -> bool {
    if err.to_string().contains("no rows") {
        return true;
    }

    for cause in err.chain() {
        if let Some(sqlx_err) = cause.downcast_ref::<sqlx::Error>() {
            if matches!(sqlx_err, sqlx::Error::RowNotFound) {
                return true;
            }
        }
    }

    false
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
    } else if err.find::<BadRequest>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: "Bad request".to_string(),
            }),
            StatusCode::BAD_REQUEST,
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
