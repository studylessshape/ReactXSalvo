use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub account: String,
    pub username: String,
    pub password: String,
    pub status: i8,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}