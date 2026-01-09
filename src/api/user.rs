use crate::{
    app::AppState,
    common::{BasePageDTO, PageInfoData},
    entity::{prelude::SysUser, sys_user},
    error::ApiResult,
    query::Query,
    response::AppResponse,
    valid::Valid,
};
use anyhow::Context;
use axum::{Router, debug_handler, extract::State, routing::get};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait,
};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/users", get(get_users))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryDTO {
    keyword: Option<String>,

    #[validate(nested)]
    #[serde(flatten)]
    pagination: BasePageDTO,
}

#[debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    Valid(Query(UserQueryDTO {
        keyword,
        pagination,
    })): Valid<Query<UserQueryDTO>>,
) -> ApiResult<AppResponse<PageInfoData<sys_user::Model>>> {
    let paginate = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Username.contains(keyword))
                    .add(sys_user::Column::Id.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedDate)
        .paginate(&db, pagination.size);
    let total = paginate.num_items().await?;
    let users = paginate.fetch_page(pagination.page - 1).await?;
    let pigination = PageInfoData::from_pagination(pagination, total, users);
    //.context("can not find users")?;
    Ok(AppResponse::ok(Some(pigination)))
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
