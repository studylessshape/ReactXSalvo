use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    /// 父级ID
    pub parent_id: Option<String>,
    /// 菜单关键字
    #[sea_orm(unique)]
    pub key: String,
    /// 菜单名称
    pub name: String,
    /// 菜单类型（1：目录，2：菜单，3：按钮）
    pub menu_type: i8,
    /// 路由地址
    pub path: String,
    /// 图标
    pub icon: Option<String>,
    /// 排序
    pub sort: u32,
    /// 菜单状态（1：正常，0：隐藏）
    pub visible: i8,
    /// 菜单状态（1：正常，0：停用）
    pub status: i8,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub create_time: DateTime,
    /// 更新时间
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}