use std::net::SocketAddr;

use crate::app::auth::{Principal, get_jwt};
use crate::app::{
    ApiError, AppResponse, AppResult, AppState, Gender, ResponseErrorCode, ValidJson,
    get_auth_layer,
};
use crate::entity::{prelude::*, sys_user};
use crate::utils::crypt;
use axum::extract::ConnectInfo;
use axum::response::Response;
use axum::{Extension, Json};
use axum::{Router, debug_handler, extract::State, routing};
use sea_orm::sea_query::ExprTrait;
use sea_orm::{ActiveValue, ColumnTrait, Condition, QueryFilter};
use sea_orm::{
    DeriveIntoActiveModel, EntityTrait,
    prelude::{Date, DateTime},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(username = %dto.username,ip=%addr.ip()))]
async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(dto): ValidJson<UserLoginDTO>,
) -> AppResult<LoginVO> {
    tracing::info!("login username:{}", &dto.username);
    let user = SysUser::find()
        .filter(sys_user::Column::Account.eq(&dto.username))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz((ResponseErrorCode::UserNameOrPasswordError)))?;

    let user_password_hash = &user.password;
    let input_password = &dto.password;
    let password_match = crypt::verify_password(input_password, user_password_hash)?;
    if !password_match {
        return Err(ApiError::Biz(ResponseErrorCode::UserNameOrPasswordError));
    }
    let principal = Principal {
        id: user.id.to_string(),
        username: user.username,
        roles: vec![],
        permissions: vec![],
    };
    let access_token = get_jwt().encode(principal)?;
    tracing::info!(" login suceessaccess_token:{}", &access_token);
    Ok(AppResponse::ok(Some(LoginVO { access_token })))
}

#[debug_handler]
async fn get_user_info(Extension(principal): Extension<Principal>) -> AppResult<Principal> {
    Ok(AppResponse::ok(Some(principal)))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserLoginDTO {
    #[validate(length(min = 6, max = 20, message = "账号长度在6-20个字符之间"))]
    pub username: String,

    #[validate(length(min = 6, max = 20))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginVO {
    access_token: String,
}
