mod api;
mod app;
mod config;
mod entity;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
