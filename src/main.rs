mod app;
mod config;
mod database;
mod entity;
mod logger;
mod server;
use crate::{
    app::AppState,
    entity::{prelude::SysUser, sys_user},
};
use axum::{Router, debug_handler, extract::State, response::IntoResponse, routing};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(get_users));
    app::run(router).await
}
#[debug_handler]
async fn index() -> &'static str {
    "hello world"
}
/**
* 获取用户列表
*/
#[debug_handler]
async fn get_users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
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
        .unwrap();
    axum::Json(users)
}
