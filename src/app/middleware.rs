use std::{pin::Pin, sync::LazyLock};

use axum::{
    body::Body,
    extract::Request,
    http::header,
    response::{IntoResponse, Response},
};
use jsonwebtoken::jwk::Jwk;
use tower_http::{
    auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer},
    limit::ResponseBody,
};

use crate::app::{
    ApiError, AppResponse,
    auth::{JWT, get_jwt},
    json::Json,
};
static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::new(JWTAuth { jwt: get_jwt() }));
#[derive(Debug, Clone)]
pub struct JWTAuth {
    pub jwt: &'static JWT,
}

impl JWTAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        Self { jwt }
    }
}

// impl AsyncAuthorizeRequest<Body> for JWTAuth {

// }
impl AsyncAuthorizeRequest<Body> for JWTAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;
    fn authorize(&mut self, mut request: axum::http::Request<Body>) -> Self::Future {
        let jwt = self.jwt;
        Box::pin(async move {
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization header is invalid format",
                            ))
                        })?
                        .strip_prefix("Bear ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization header Bear is missing",
                            ))
                        });
                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| ApiError::Unauthenticated(String::from("")))??;
            let principal = jwt
                .decode(token)
                .map_err(|err| ApiError::InternalServerError(err))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

impl From<ApiError> for Response {
    fn from(error: ApiError) -> Self {
        error.into_response()
    }
}
pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
    &AUTH_LAYER
}
