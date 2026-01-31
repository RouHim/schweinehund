mod config;
mod error;
mod handlers_static;
mod handlers_task;
mod handlers_zone;
mod models;
mod ntfy;
mod scheduler;
mod state;
mod storage;

use std::env;
use std::net::SocketAddr;

use axum::extract::{Path, State};
use axum::http::header;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, patch, post};
use axum::{Extension, Router};
use include_dir::{include_dir, Dir};

use crate::ntfy::NtfyClient;
use crate::state::{AppState, SharedState};

static STATIC_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let port = env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8090);

    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    let ntfy_url = config::ntfy_url();

    let state: SharedState = match AppState::new(&data_dir) {
        Ok(state) => std::sync::Arc::new(state),
        Err(err) => {
            tracing::error!(error = %err, "failed to initialize app state");
            return;
        }
    };

    let ntfy = NtfyClient::new(&ntfy_url);
    ntfy.send("Backend started", "Schweinehund", 2, "dog").await;

    scheduler::start_scheduler(state.clone(), ntfy.clone());

    let app = Router::new()
        .route("/api/health", get(handlers_static::health_handler))
        .route(
            "/api/tasks",
            get(handlers_task::get_tasks).post(handlers_task::create_task),
        )
        .route(
            "/api/tasks/:id",
            patch(handlers_task::update_task).delete(handlers_task::delete_task),
        )
        .route(
            "/api/zones",
            get(handlers_zone::get_zones).post(handlers_zone::create_zone),
        )
        .route(
            "/api/zones/:id",
            patch(handlers_zone::update_zone).delete(handlers_zone::delete_zone),
        )
        .route("/partials/:name", get(handlers_static::partials_handler))
        .route("/api/admin/cron/daily", post(trigger_daily))
        .route("/api/admin/cron/weekly", post(trigger_weekly))
        .route("/", get(index_handler))
        .route("/*path", get(static_handler))
        .with_state(state)
        .layer(Extension(&STATIC_DIR))
        .layer(Extension(ntfy));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "listening");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!(error = %err, "failed to bind");
            return;
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(error = %err, "server error");
    }
}

async fn trigger_daily(
    State(state): State<SharedState>,
    Extension(ntfy): Extension<NtfyClient>,
) -> impl IntoResponse {
    scheduler::run_daily_once(&state, &ntfy).await;
    StatusCode::NO_CONTENT
}

async fn trigger_weekly(
    State(state): State<SharedState>,
    Extension(ntfy): Extension<NtfyClient>,
) -> impl IntoResponse {
    scheduler::run_weekly_once(&state, &ntfy).await;
    StatusCode::NO_CONTENT
}

async fn index_handler(Extension(dir): Extension<&'static Dir<'static>>) -> Response {
    serve_static_file(dir, "index.html")
}

async fn static_handler(
    Extension(dir): Extension<&'static Dir<'static>>,
    Path(path): Path<String>,
) -> Response {
    let path = if path.is_empty() {
        "index.html"
    } else {
        path.as_str()
    };
    serve_static_file(dir, path)
}

fn serve_static_file(dir: &'static Dir<'static>, path: &str) -> Response {
    let path = path.trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    let file = match dir.get_file(path) {
        Some(file) => file,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let content_type = match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    };

    let mut resp = file.contents().into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(content_type),
    );
    resp
}
