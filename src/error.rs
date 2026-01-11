use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::response::AppResponse;
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("method not allowed")]
    MethodNotAllowed,

    #[error("biz error:{0:?} - {message}", message = ".0.message()")]
    Biz(ResponseErrorCode),

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

    #[error("bcrypt error:{0}")]
    Bcrpt(#[from] bcrypt::BcryptError),
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
            ApiError::InternalServerError(_) | ApiError::DatabaseError(_) | ApiError::Bcrpt(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::ParameterError(_)
            | ApiError::PathError(_)
            | ApiError::JsonError(_)
            | ApiError::ValidationError(_)
            | ApiError::Biz { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        tracing::error!("{}", &self);

        let body = match &self {
            ApiError::Biz(error_code) => axum::Json(AppResponse::<()>::fail(
                error_code.code() as i32,
                error_code.message(),
            )),
            _ => axum::Json(AppResponse::<()>::fail_enum(&self)),
        };
        (status_code, body).into_response()
    }
}

macro_rules! define_error_codes {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident($code:expr, $msg:expr)),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn code(&self) -> u16 {
                match self {
                    $(Self::$variant => $code,)*
                }
            }

            pub fn message(&self) -> &'static str {
                match self {
                    $(Self::$variant => $msg,)*
                }
            }

            // Generate associated constants
            $(
                pub const $variant: Self = Self::$variant;
            )*
        }
    };
}
define_error_codes! {
     #[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
    pub enum ResponseErrorCode {
        FindNotUser(5001, "找不到用户"),
        DB_PWD_NOT_FIND(5002, "数据库密码未找到")
        // Add more error codes as needed
    }
}
