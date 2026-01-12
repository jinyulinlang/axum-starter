mod user;

use axum::Router;

use crate::app::ApiResult;
use crate::app::{ApiError, AppState};

/// Create the router for the API.
///
/// This function creates a new router and merges the user router into it.
///
/// The returned router is ready to be used by the axum server.
pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api", Router::new().nest("/users", user::create_router()))
        .fallback(handler_not_found)
        .method_not_allowed_fallback(handler_method_not_allowed)
}

async fn handler_not_found() -> ApiResult<()> {
    tracing::warn!("Not Found");
    Err(ApiError::NotFound)
}

async fn handler_method_not_allowed() -> ApiResult<()> {
    tracing::warn!("Method Not Allowed");
    Err(ApiError::MethodNotAllowed)
}
