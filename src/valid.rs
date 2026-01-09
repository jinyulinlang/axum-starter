use axum::extract::{FromRequest, FromRequestParts};
use axum_valid::HasValidate;
use serde::de;
use validator::Validate;

use crate::{error::ApiError, json::Json, path::Path, query::Query};

#[derive(Debug, Clone, Default, FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct Valid<T>(pub T);

impl<T> HasValidate for Valid<T> {
    type Validate = T;
    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValidQuery<T>(pub T);

#[derive(Debug, Clone, Default)]
pub struct ValidPath<T>(pub T);
pub struct ValidJson<T>(pub T);
impl<S, T> FromRequestParts<S> for ValidPath<T>
where
    S: Send + Sync,
    Path<T>: FromRequestParts<S, Rejection = ApiError> + axum_valid::HasValidate,
    <Path<T> as axum_valid::HasValidate>::Validate: Validate,
{
    type Rejection = ApiError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let path = axum_valid::Valid::<Path<T>>::from_request_parts(parts, state).await?;
        Ok(ValidPath(path.0.0))
    }
}
impl<S, T> FromRequestParts<S> for ValidQuery<T>
where
    S: Send + Sync,
    Query<T>: FromRequestParts<S, Rejection = ApiError> + axum_valid::HasValidate,
    <Query<T> as axum_valid::HasValidate>::Validate: Validate,
{
    type Rejection = ApiError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let query = axum_valid::Valid::<Query<T>>::from_request_parts(parts, state).await?;
        Ok(ValidQuery(query.0.0))
    }
}
