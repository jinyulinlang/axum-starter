use axum::{
    extract::rejection::{self, JsonRejection, PathRejection, QueryRejection},
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
    #[error("database exception:{0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Internal Server Error {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error("query param error:{0}")]
    ParameterError(#[from] QueryRejection),
    #[error("path param error:{0}")]
    PathError(#[from] PathRejection),
    #[error("body param error:{0}")]
    JsonError(#[from] JsonRejection),

    #[error("validation error:{0}")]
    ValidationError(String),
}
impl From<axum_valid::ValidRejection<ApiError>> for ApiError {
    fn from(rejection: axum_valid::ValidRejection<ApiError>) -> Self {
        match rejection {
            axum_valid::ValidationRejection::Valid(err) => {
                ApiError::ValidationError(err.to_string())
            }
            axum_valid::ValidationRejection::Inner(err) => err,
        }
    }
}
impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::InternalServerError(_) | ApiError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::ParameterError(_)
            | ApiError::PathError(_)
            | ApiError::JsonError(_)
            | ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
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
