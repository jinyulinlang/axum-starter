use crate::{
    app::AppResponse,
    app::AppResult,
    app::AppState,
    app::Gender,
    app::Path,
    app::{ApiError, ApiResult, ResponseErrorCode},
    app::{BasePageDTO, PageInfoData},
    app::{ValidJson, ValidQuery},
    entity::{prelude::SysUser, sys_user},
};
use anyhow::Context;
use axum::{
    Router, debug_handler,
    extract::State,
    routing::{delete, get, post, put},
};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder, QueryTrait, prelude::Date,
};
use sea_orm::{IntoActiveModel as _, prelude::*};
use serde::Deserialize;
use sys_user::ActiveModel;
use validator::Validate;
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/pagination", post(find_page))
        .route("/", get(get_users))
        .route("/", post(add_user))
        .route("/", put(update_user))
        .route("/{id}", delete(delete_user))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryDTO {
    keyword: Option<String>,

    #[validate(nested)]
    #[serde(flatten)]
    pagination: BasePageDTO,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserUpdateDTO {
    // not emepty
    #[validate(length(min = 1, message = "id不能为空"))]
    pub id: String,

    #[validate(nested)]
    #[serde(flatten)]
    pub user: UserAddDTO,
}
#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct UserAddDTO {
    #[validate(length(min = 2, max = 20, message = "用户名长度在2-20个字符之间"))]
    pub username: String,

    // #[validate(length(min = 1, max = 1))]
    pub gender: Gender,

    #[validate(length(min = 1, max = 20, message = "账号长度在6-20个字符之间"))]
    pub account: String,

    #[validate(length(min = 6, max = 20))]
    pub password: String,

    #[validate(custom(function = "crate::app::is_mobile_phone"))]
    pub mobile_phone: String,

    pub birthday: Date,

    pub enbaled: bool,
}

#[debug_handler]
async fn add_user(
    State(AppState { db }): State<AppState>,
    ValidJson(dto): ValidJson<UserAddDTO>,
) -> AppResult<()> {
    let mut active_model = dto.into_active_model();
    active_model.password = sea_orm::ActiveValue::Set(bcrypt::hash(
        &active_model
            .password
            .take()
            .ok_or_else(|| ApiError::Biz(ResponseErrorCode::DbPwdNotFind))?,
        bcrypt::DEFAULT_COST,
    )?);
    let _am = active_model.insert(&db).await?;
    Ok(AppResponse::ok_whitok_no_data())
}
#[debug_handler]
async fn update_user(
    State(AppState { db }): State<AppState>,
    ValidJson(dto): ValidJson<UserUpdateDTO>,
) -> AppResult<()> {
    let existed_user = SysUser::find_by_id(&dto.id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(ResponseErrorCode::FindNotUser))?;
    let old_password = existed_user.password.clone();
    let password = dto.user.password.clone();
    let mut existed_user_model = existed_user.into_active_model();

    let mut active_model = dto.user.into_active_model();
    existed_user_model.clone_from(&active_model);
    // existed_user_model.id = ActiveValue::Unchanged(id);
    if password.is_empty() {
        active_model.password = ActiveValue::Unchanged(old_password);
    } else {
        let password_value = &active_model
            .password
            .take()
            .ok_or_else(|| ApiError::Biz(ResponseErrorCode::DbPwdNotFind))?;
        existed_user_model.password =
            ActiveValue::Set(bcrypt::hash(password_value, bcrypt::DEFAULT_COST)?);
    }
    let _ret = active_model.update(&db).await?;
    Ok(AppResponse::ok_whitok_no_data())
}

#[debug_handler]
async fn delete_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<()> {
    let existed_user = SysUser::find_by_id(&id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(ResponseErrorCode::FindNotUser))?;
    let result = existed_user.delete(&db).await?;
    tracing::info!(
        "delete user: {},affected rows: {}",
        id,
        result.rows_affected
    );
    Ok(AppResponse::ok_whitok_no_data())
}
#[debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryDTO {
        keyword,
        pagination,
    }): ValidQuery<UserQueryDTO>,
) -> AppResult<PageInfoData<sys_user::Model>> {
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
