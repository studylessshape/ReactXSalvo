use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_menu_element")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub menu_id: Uuid,
    pub key: String,
    pub name: String,
    /// 状态（1：正常，0：隐藏）
    pub visible: i32,
    /// 状态（1：正常，0：停用）
    pub status: i32,
    pub remark: Option<String>,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Menu,
    RoleMenuElement,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Menu => Entity::belongs_to(super::menu::Entity)
                .from(Column::MenuId)
                .to(super::menu::Column::Id)
                .into(),
            Self::RoleMenuElement => Entity::has_many(super::role_menu_element::Entity).into(),
        }
    }
}

impl Related<super::menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Menu.def()
    }
}

impl Related<super::role_menu_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleMenuElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
