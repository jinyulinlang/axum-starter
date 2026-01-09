use crate::{
    app::AppState,
    entity::{prelude::SysUser, sys_user},
    error::ApiResult,
    response::AppResponse,
};
use anyhow::Context;
use axum::{Router, debug_handler, extract::State, routing::get};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, sea_query::Mode};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/users", get(get_users))
}

#[debug_handler]
// #[tracing::instrument(name = "get_users", fields(pay_method = "alipay"), skip(db))]
async fn get_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<AppResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("male"))
                .add(
                    sys_user::Column::Username
                        .starts_with("张")
                        .add(Condition::any().add(sys_user::Column::Enbaled.eq(true))),
                ),
        )
        .all(&db)
        .await
        .context("can not find users")?;
    Ok(AppResponse::ok(Some(users)))
}
