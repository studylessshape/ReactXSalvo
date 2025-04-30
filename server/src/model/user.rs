use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub account: String,
    pub username: String,
    pub password: String,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}