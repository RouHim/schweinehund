use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use warp::Filter;

mod assets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    let static_files = assets::serve_embedded();

    let routes = health.or(static_files);

    let addr: SocketAddr = "127.0.0.1:3000".parse()?;
    tracing::info!("Starting server on {}", addr);

    let (_, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(addr, shutdown_signal());

    server.await;
    tracing::info!("Server shutdown gracefully");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    tracing::info!("Received CTRL+C signal");
}
