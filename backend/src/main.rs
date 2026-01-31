mod config;
mod state;
mod storage;
mod models;
mod error;
mod handlers_task;
mod handlers_zone;
mod handlers_static;
mod scheduler;
mod ntfy;

#[tokio::main]
async fn main() {
    println!("Schweinehund backend starting...");
}
