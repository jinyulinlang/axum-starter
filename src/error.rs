use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response::AppResponse;
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("method not allowed")]
    MethodNotAllowed,

    #[allow(dead_code)]
    #[error("{0}")]
    Biz(String),

    #[error("Internal Server Error {0}")]
    InternalServerError(#[from] anyhow::Error),
}
impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        tracing::error!("{}", &self);
        let body = axum::Json(AppResponse::<()>::fail_enum(&self));
        (status_code, body).into_response()
    }
}
