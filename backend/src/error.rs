use std::convert::Infallible;

use serde::Serialize;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl warp::reject::Reject for ApiError {}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

pub fn bad_request(message: impl Into<String>) -> Rejection {
    warp::reject::custom(ApiError {
        status: StatusCode::BAD_REQUEST,
        message: message.into(),
    })
}

pub fn not_found(message: impl Into<String>) -> Rejection {
    warp::reject::custom(ApiError {
        status: StatusCode::NOT_FOUND,
        message: message.into(),
    })
}

pub fn internal(message: impl Into<String>) -> Rejection {
    warp::reject::custom(ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: message.into(),
    })
}

pub async fn recover(rej: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(err) = rej.find::<ApiError>() {
        let body = warp::reply::json(&ErrorBody {
            error: err.message.clone(),
        });
        return Ok(warp::reply::with_status(body, err.status));
    }

    if rej.is_not_found() {
        let body = warp::reply::json(&ErrorBody {
            error: "not found".to_string(),
        });
        return Ok(warp::reply::with_status(body, StatusCode::NOT_FOUND));
    }

    if rej
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        let body = warp::reply::json(&ErrorBody {
            error: "invalid json".to_string(),
        });
        return Ok(warp::reply::with_status(body, StatusCode::BAD_REQUEST));
    }

    let body = warp::reply::json(&ErrorBody {
        error: "internal server error".to_string(),
    });
    Ok(warp::reply::with_status(
        body,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
