mod config;
mod database;
mod entity;
mod logger;
use anyhow::Ok;
use axum::{Router, debug_handler, extract::State, response::IntoResponse, routing};
use entity::prelude::*;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::net::TcpListener;
use tracing::info;

use crate::entity::sys_user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let db = database::init().await?;
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(get_users))
        .with_state(db);
    let port = config::get().server().port();
    let lister = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("Listening on http://0.0.0.0:{port}");
    axum::serve(lister, router).await?;
    Ok(())
}
#[debug_handler]
async fn index() -> &'static str {
    "hello world"
}

#[debug_handler]
async fn get_users(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users = SysUser::find()
        // 过滤出性别为男性的用户
        // .filter(sys_user::Column::Gender.eq("male"))
        // 添加多个过滤条件
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("male"))
                .add(sys_user::Column::Age.gt(18))
                .add(
                    Condition::any()
                        // .add(sys_user::Column::Birthday.between(Date, b))
                        .add(sys_user::Column::Enbaled.eq(true)),
                ),
        )
        .all(&db)
        .await?;
    Ok(axum::Json(users))
}
