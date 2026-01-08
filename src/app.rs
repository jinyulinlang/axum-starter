use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{database, logger, server::Server};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting server...");
    let db = database::init().await?;
    let state = AppState::new(db);
    let server_config = crate::config::get().server();
    let server = Server::new(server_config);
    server.start(state, router).await
}
