use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::app::Gender;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "sys_user")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub username: String,
    pub gender: Gender,
    pub account: String,

    // not serialize （返回前端的时候不显示 password）
    #[serde(skip_serializing)]
    pub password: String,

    pub mobile_phone: String,
    pub birthday: Date,
    pub enbaled: bool,
    pub created_date: DateTime,
    pub updated_date: DateTime,
    pub created_by: String,
    pub updated_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = Set(crate::utils::id::next_id());
        }
        Ok(self)
    }
}
