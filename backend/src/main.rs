mod config;
mod db;
mod error;
mod handlers_static;
mod handlers_task;
mod handlers_zone;
mod ntfy;
mod scheduler;

use std::convert::Infallible;

use include_dir::{include_dir, Dir};
use sqlx::SqlitePool;
use warp::filters::path::FullPath;
use warp::http::header;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::Response;
use warp::Filter;

use crate::ntfy::NtfyClient;

static STATIC_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let port = config::port();
    let database_url = config::database_url();
    let ntfy_url = config::ntfy_url();

    let pool = match db::create_pool(&database_url).await {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!(error = %err, "failed to connect sqlite");
            return;
        }
    };

    if let Err(err) = db::run_migrations(&pool).await {
        tracing::error!(error = %err, "failed to run migrations");
        return;
    }

    let ntfy = NtfyClient::new(&ntfy_url);
    ntfy.send("Backend started", "Schweinehund", 2, "dog").await;

    scheduler::start_scheduler(pool.clone(), ntfy.clone());

    let with_pool = with_pool(pool);
    let with_ntfy = with_ntfy(ntfy);
    let with_static_dir = with_static_dir();

    let health = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handlers_static::health);

    let get_tasks = warp::path!("api" / "tasks")
        .and(warp::get())
        .and(with_pool.clone())
        .and_then(handlers_task::get_tasks);

    let create_task = warp::path!("api" / "tasks")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool.clone())
        .and_then(handlers_task::create_task);

    let patch_task = warp::path!("api" / "tasks" / String)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_pool.clone())
        .and_then(handlers_task::update_task);

    let delete_task = warp::path!("api" / "tasks" / String)
        .and(warp::delete())
        .and(with_pool.clone())
        .and_then(handlers_task::delete_task);

    let tasks = get_tasks.or(create_task).or(patch_task).or(delete_task);

    let get_zones = warp::path!("api" / "zones")
        .and(warp::get())
        .and(with_pool.clone())
        .and_then(handlers_zone::get_zones);

    let create_zone = warp::path!("api" / "zones")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool.clone())
        .and_then(handlers_zone::create_zone);

    let patch_zone = warp::path!("api" / "zones" / String)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_pool.clone())
        .and_then(handlers_zone::update_zone);

    let delete_zone = warp::path!("api" / "zones" / String)
        .and(warp::delete())
        .and(with_pool.clone())
        .and_then(handlers_zone::delete_zone);

    let zones = get_zones.or(create_zone).or(patch_zone).or(delete_zone);

    let partials = warp::path!("partials" / String)
        .and(warp::get())
        .and(with_static_dir.clone())
        .and_then(handlers_static::partial);

    let trigger_daily = warp::path!("api" / "admin" / "cron" / "daily")
        .and(warp::post())
        .and(with_pool.clone())
        .and(with_ntfy.clone())
        .and_then(trigger_daily);

    let trigger_weekly = warp::path!("api" / "admin" / "cron" / "weekly")
        .and(warp::post())
        .and(with_pool.clone())
        .and(with_ntfy.clone())
        .and_then(trigger_weekly);

    let static_files = warp::get()
        .and(warp::path::full())
        .and(with_static_dir)
        .and_then(serve_static);

    let routes = health
        .or(tasks)
        .or(zones)
        .or(partials)
        .or(trigger_daily)
        .or(trigger_weekly)
        .or(static_files)
        .recover(error::recover);

    tracing::info!(port, "listening");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

fn with_pool(pool: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_ntfy(ntfy: NtfyClient) -> impl Filter<Extract = (NtfyClient,), Error = Infallible> + Clone {
    warp::any().map(move || ntfy.clone())
}

fn with_static_dir() -> impl Filter<Extract = (&'static Dir<'static>,), Error = Infallible> + Clone
{
    warp::any().map(|| &STATIC_DIR)
}

async fn trigger_daily(pool: SqlitePool, ntfy: NtfyClient) -> Result<impl warp::Reply, Infallible> {
    scheduler::run_daily_once(&pool, &ntfy).await;
    Ok(warp::reply::with_status(
        warp::reply(),
        StatusCode::NO_CONTENT,
    ))
}

async fn trigger_weekly(
    pool: SqlitePool,
    ntfy: NtfyClient,
) -> Result<impl warp::Reply, Infallible> {
    scheduler::run_weekly_once(&pool, &ntfy).await;
    Ok(warp::reply::with_status(
        warp::reply(),
        StatusCode::NO_CONTENT,
    ))
}

async fn serve_static(path: FullPath, dir: &'static Dir<'static>) -> Result<Response, Infallible> {
    let mut req_path = path.as_str().trim_start_matches('/').to_string();
    if req_path.is_empty() {
        req_path = "index.html".to_string();
    }

    let file = match dir.get_file(&req_path) {
        Some(file) => file,
        None => {
            if req_path == "index.html" {
                let mut resp = Response::new(Body::empty());
                *resp.status_mut() = StatusCode::NOT_FOUND;
                return Ok(resp);
            }
            // SPA-ish fallback
            match dir.get_file("index.html") {
                Some(file) => file,
                None => {
                    let mut resp = Response::new(Body::empty());
                    *resp.status_mut() = StatusCode::NOT_FOUND;
                    return Ok(resp);
                }
            }
        }
    };

    let content_type = match req_path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    };

    let mut resp = Response::new(Body::from(file.contents().to_vec()));
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(content_type),
    );
    Ok(resp)
}
