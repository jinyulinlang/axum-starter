mod common;
mod database;
mod enumeration;
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
mod validation;

pub use enumeration::Gender;
pub use error::ApiError;
pub use response::AppResponse;

pub use common::BasePageDTO;
pub use common::PageInfoData;
pub use error::ResponseErrorCode;
pub use path::Path;
pub use query::Query;
pub use serde::deserialize_number;
pub use valid::Valid;
pub use valid::ValidJson;
pub use valid::ValidPath;
pub use valid::ValidQuery;
pub use validation::is_mobile_phone;

use axum::Router;
use sea_orm::DatabaseConnection;

use crate::app::server::Server;

pub(crate) type ApiResult<T> = Result<T, ApiError>;

pub(crate) type AppResult<T> = ApiResult<AppResponse<T>>;

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
    // init id generator
    crate::utils::id::init()?;
    let db = database::init().await?;
    let state = AppState::new(db);
    let server_config = crate::config::get().server();
    let server = Server::new(server_config);
    server.start(state, router).await
}
