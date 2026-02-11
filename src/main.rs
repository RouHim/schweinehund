use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use warp::Filter;

use schweinehund::assets;
use schweinehund::db;
use schweinehund::routes;
use schweinehund::scheduler;

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:schweinehund.db".to_string());

    tracing::info!("Initializing database at {}", database_url);
    let pool = db::init_pool(&database_url).await?;

    tracing::info!("Running database migrations");
    db::run_migrations(&pool).await?;

    tracing::info!("Starting weekly reset scheduler");
    let _scheduler_handle = scheduler::start_scheduler(pool.clone());

    let api = routes::api_routes(pool);
    let static_files = assets::serve_embedded();

    let all_routes = static_files.or(api).recover(routes::handle_rejection);

    let addr: SocketAddr = "0.0.0.0:9666".parse()?;
    tracing::info!("Starting server on {}", addr);

    warp::serve(all_routes)
        .bind(addr)
        .await
        .graceful(shutdown_signal())
        .run()
        .await;

    tracing::info!("Server shutdown gracefully");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    tracing::info!("Received CTRL+C signal");
}
