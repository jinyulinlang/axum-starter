use sea_orm::{IntoActiveValue, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    #[sea_orm(string_value = "male")]
    // #[serde(rename = "1")]
    // #[sea_orm(string_value = "1")]
    // 上述两种写法都可以
    Male,
    #[sea_orm(string_value = "female")]
    Female,
}

impl IntoActiveValue<Gender> for Gender {
    fn into_active_value(self) -> sea_orm::ActiveValue<Gender> {
        sea_orm::ActiveValue::Set(self)
    }
}
