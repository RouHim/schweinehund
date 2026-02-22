use sqlx::SqlitePool;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

mod debug;
mod deep_cleaning;
mod errors;
mod health;
mod import;
mod settings;
mod tasks;
mod types;
mod validation;

pub use errors::handle_rejection;

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

    health::routes()
        .or(tasks::routes(pool.clone()))
        .or(deep_cleaning::routes(pool.clone()))
        .or(settings::routes(pool.clone()))
        .or(debug::routes(pool.clone()))
        .or(import::routes(pool))
        .with(cors)
}
