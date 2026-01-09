mod api;
mod app;
mod common;
mod config;
mod database;
mod entity;
mod error;
mod json;
mod latency;
mod logger;
mod path;
mod query;
mod response;
mod serde;
mod server;
mod valid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
