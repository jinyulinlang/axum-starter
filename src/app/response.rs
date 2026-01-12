use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::app::error::ApiError;

const SUCESSS_CODE: i32 = 200;
const SUCESSS_MESSAGE: &str = "success";
#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponse<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
}

impl<T> AppResponse<T> {
    pub fn new(code: i32, data: Option<T>, message: String) -> Self {
        Self {
            code,
            data,
            message,
        }
    }
    pub fn ok(data: Option<T>) -> Self {
        AppResponse::new(SUCESSS_CODE, data, SUCESSS_MESSAGE.to_string())
    }
    #[allow(dead_code)]
    pub fn ok_whitok_no_data() -> Self {
        AppResponse::ok(None)
    }

    pub fn fail<M: AsRef<str>>(code: i32, message: M) -> Self {
        AppResponse::new(code, None, String::from(message.as_ref()))
    }

    #[allow(dead_code)]
    pub fn fail_with_u16<M: AsRef<str>>(code: u16, message: M) -> Self {
        let status_code = code as i32;
        AppResponse::fail(status_code, message)
    }
    pub fn fail_enum(api_error: &ApiError) -> Self {
        AppResponse::fail(
            api_error.status_code().as_u16() as i32,
            api_error.to_string(),
        )
    }
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
