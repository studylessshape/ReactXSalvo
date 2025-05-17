use sea_orm::entity::prelude::*;
use sea_orm_ext::{InsertActiveModel, InsertModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, DeriveEntityModel, InsertActiveModel)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub account: String,
    pub username: String,
    pub password: String,
    #[sea_orm(default_value = 1)]
    pub status: i32,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, InsertModel)]
pub struct NewUser {
    pub id: Uuid,
    pub account: String,
    pub username: String,
    pub password: String,
    pub create_time: DateTime,
}