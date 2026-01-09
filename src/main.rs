mod api;
mod app;
mod common;
mod config;
mod database;
mod entity;
mod error;
mod latency;
mod logger;
mod response;
mod serde;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
