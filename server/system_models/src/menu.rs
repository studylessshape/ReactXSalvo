use sea_orm::{entity::prelude::*};
use sea_orm_ext::{InsertActiveModel, InsertModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel, InsertActiveModel)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 父级ID
    pub parent_id: Option<Uuid>,
    /// 菜单关键字
    #[sea_orm(unique)]
    pub key: String,
    /// 菜单名称
    pub name: String,
    /// 菜单类型（1：目录，2：菜单，3：按钮）
    pub menu_type: i32,
    /// 路由地址
    pub path: String,
    /// 图标
    pub icon: Option<String>,
    /// 排序
    pub sort: i32,
    /// 菜单状态（1：正常，0：隐藏）
    #[sea_orm(default_value = 1)]
    pub visible: i32,
    /// 菜单状态（1：正常，0：停用）
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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MenuElement,
    RoleMenu,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::MenuElement => Entity::has_many(super::menu_element::Entity).into(),
            Relation::RoleMenu => Entity::has_many(super::role_menu::Entity).into(),
        }
    }
}

impl Related<super::menu_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MenuElement.def()
    }
}

impl Related<super::role_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleMenu.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, InsertModel)]
pub struct InsertModel {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub key: String,
    pub name: String,
    pub menu_type: i32,
    pub path: String,
    pub icon: Option<String>,
    pub sort: i32,
    pub remark: Option<String>,
    pub create_time: DateTime,
}