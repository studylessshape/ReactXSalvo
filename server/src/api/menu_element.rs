use salvo::{
    oapi::{endpoint, extract::*},
    prelude::*,
};
use sea_orm::{
    prelude::DateTime, sqlx::types::chrono, ActiveValue::Set, ColumnTrait as _, DerivePartialModel,
    EntityTrait as _, FromQueryResult, QueryFilter as _,
};
use sea_orm_ext::{InsertActiveModelTrait, InsertModelTrait};
use serde::{Deserialize, Serialize};
use system_models::menu_element;
use uuid::Uuid;

use crate::{db, error::AppError, json_ok, JsonResult};

use super::{Id, Ids};

#[derive(Deserialize, Debug, ToSchema)]
struct AddMenuElement {
    pub menu_id: Uuid,
    pub key: String,
    pub name: String,
}

#[endpoint(tags("Menu Element"))]
pub async fn add_menu_element(
    _token: CookieParam<String, true>,
    data: JsonBody<AddMenuElement>,
) -> JsonResult<Id<Uuid>> {
    let AddMenuElement { menu_id, key, name } = data.into_inner();
    let conn = db::conn().await?;
    if menu_element::Entity::find()
        .filter(menu_element::Column::Key.eq(&key))
        .filter(menu_element::Column::MenuId.eq(menu_id))
        .one(&conn)
        .await?
        .is_some()
    {
        return Err(AppError::public("Menu element already exists"));
    }

    let new_element = menu_element::InsertModel {
        id: Uuid::now_v7(),
        menu_id,
        key,
        name,
        create_time: chrono::Local::now().naive_utc(),
    };

    let new_element: menu_element::Model = new_element.insert(&conn).await?;
    json_ok(Id { id: new_element.id })
}

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "menu_element::Entity")]
pub struct MenuElementModel {
    pub id: Uuid,
    pub menu_id: Uuid,
    pub key: String,
    pub name: String,
    pub visible: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
}

impl From<menu_element::Model> for MenuElementModel {
    fn from(value: menu_element::Model) -> Self {
        let menu_element::Model {
            id,
            menu_id,
            key,
            name,
            visible,
            status,
            remark,
            create_time,
            update_time,
            ..
        } = value;
        Self {
            id,
            menu_id,
            key,
            name,
            visible,
            status,
            remark,
            create_time,
            update_time,
        }
    }
}

#[endpoint(tags("Menu Element"))]
pub async fn get_menu_element(
    _token: CookieParam<String, true>,
    menu_id: QueryParam<Uuid, true>,
) -> JsonResult<Vec<MenuElementModel>> {
    let menu_id = menu_id.into_inner();
    let conn = db::conn().await?;
    let elements = menu_element::Entity::find()
        .filter(menu_element::Column::MenuId.eq(menu_id))
        .all(&conn)
        .await?;
    json_ok(elements.into_iter().map(|e| e.into()).collect())
}

#[endpoint(tags("Menu Element"))]
pub async fn update_menu_element(
    _token: CookieParam<String, true>,
    data: JsonBody<MenuElementModel>,
) -> JsonResult<()> {
    let MenuElementModel {
        id,
        menu_id,
        key,
        name,
        visible,
        status,
        remark,
        ..
    } = data.into_inner();
    let conn = db::conn().await?;
    if menu_element::Entity::find()
        .filter(menu_element::Column::MenuId.eq(menu_id))
        .filter(menu_element::Column::Key.eq(&key))
        .filter(menu_element::Column::Id.ne(id))
        .one(&conn)
        .await?
        .is_some()
    {
        return Err(AppError::public("Menu element already exists"));
    }
    let model = menu_element::ActiveModel {
        id: Set(id),
        menu_id: Set(menu_id),
        key: Set(key),
        name: Set(name),
        visible: Set(visible),
        status: Set(status),
        remark: Set(remark),
        update_time: Set(Some(chrono::Local::now().naive_utc())),
        ..Default::default()
    };

    model.insert_active(&conn).await?;
    json_ok(())
}

#[endpoint(tags("Menu Element"))]
pub async fn delete_menu_element(
    _token: CookieParam<String, true>,
    id: JsonBody<Ids<Uuid>>,
) -> JsonResult<()> {
    let ids = id.into_inner().ids;
    let conn = db::conn().await?;
    menu_element::Entity::delete_many().filter(menu_element::Column::Id.is_in(ids)).exec(&conn).await?;
    json_ok(())
}
