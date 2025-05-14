use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_role_menu_element")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub menu_element_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
    MenuElement,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::RoleId)
                .to(super::role::Column::Id)
                .into(),
            Self::MenuElement => Entity::belongs_to(super::menu_element::Entity)
                .from(Column::MenuElementId)
                .to(super::menu_element::Column::Id)
                .into(),
        }
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::menu_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MenuElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
