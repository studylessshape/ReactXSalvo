use sea_orm_ext::InsertActiveModel;
use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, InsertActiveModel)]
#[sea_orm(table_name = "sys_role")]
pub struct Model {
    /// 主键 ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 角色权限关键字
    #[sea_orm(unique)]
    pub key: String,
    /// 角色名称
    pub name: String,
    /// 角色状态（1：正常 2：停用）
    #[sea_orm(default_value = 1)]
    pub status: i32,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub create_time: DateTime,
    /// 更新时间
    pub update_time: Option<DateTime>,
    #[sea_orm(default_value = false)]
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}