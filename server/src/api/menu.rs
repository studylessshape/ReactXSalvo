use salvo::oapi::extract::*;
use salvo::prelude::*;
use sea_orm::{prelude::*, sqlx::types::chrono, ActiveValue::Set, FromQueryResult};
use serde::{Deserialize, Serialize};
use system_models::menu;
use uuid::Uuid;
use sea_orm_ext::InsertModelTrait;

use crate::{db, error::AppError, json_ok, JsonResult};

#[derive(Deserialize, ToSchema, Default, Debug)]
struct AddMenuInData {
    parent_id: Option<Uuid>,
    key: String,
    name: String,
    menu_type: i32,
    path: String,
    icon: Option<String>,
    sort: u32,
    remark: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "system_models::menu::Entity")]
struct MenuId {
    id: Uuid,
}

#[endpoint(tags("Menu"))]
pub async fn add_menu(_token: CookieParam<String, true>, data: JsonBody<AddMenuInData>) -> JsonResult<MenuId> {
    let AddMenuInData {
        parent_id,
        key,
        name,
        menu_type,
        path,
        icon,
        sort,
        remark,
    } = data.into_inner();
    let conn = db::conn().await?;
    if menu::Entity::find().filter(menu::Column::Key.eq(key.clone())).into_partial_model::<MenuId>().one(&conn).await?.is_some() {
        return Err(AppError::Public("菜单关键字已存在".to_string()));
    }

    let insert_model = menu::InsertModel {
        id: Uuid::now_v7(),
        parent_id,
        key,
        name,
        menu_type,
        path,
        icon,
        sort,
        remark,
        create_time: chrono::Local::now().naive_utc(),
    };

    let menu::Model { id, ..} = insert_model.insert(&conn).await?;
    json_ok(MenuId { id })
}

#[derive(Deserialize, Serialize, ToSchema, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "system_models::menu::Entity")]
pub struct MenuModel {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub key: String,
    pub name: String,
    pub menu_type: i32,
    pub path: String,
    pub icon: Option<String>,
    pub sort: u32,
    pub visible: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
}

impl From<menu::Model> for MenuModel {
    fn from(value: menu::Model) -> Self {
        let menu::Model { id, parent_id, key,name,menu_type,path,icon,sort,visible,status,remark,create_time,update_time,..} = value;
        Self {
            id,
            parent_id,
            key,
            name,
            menu_type,
            path,
            icon,
            sort,
            visible,
            status,
            remark,
            create_time,
            update_time,
        }
    }
}

#[endpoint(tags("Menu"))]
pub async fn update_menu(_token: CookieParam<String, true>, data: JsonBody<MenuModel>) -> JsonResult<()> {
    let MenuModel { id, parent_id, key, name, menu_type, path, icon, sort, remark,.. } = data.into_inner();

    let conn = db::conn().await?;
    if menu::Entity::find().filter(menu::Column::Key.eq(key.clone())).filter(menu::Column::Id.ne(id)).into_partial_model::<MenuId>().one(&conn).await?.is_some() {
        return Err(AppError::Public("菜单关键字已存在".to_string()));
    }

    let update_model = menu::ActiveModel {
        parent_id: Set(parent_id),
        key: Set(key),
        name: Set(name),
        menu_type: Set(menu_type),
        path: Set(path),
        icon: Set(icon),
        sort: Set(sort),
        remark: Set(remark),
        ..Default::default()
    };

    update_model.update(&conn).await?;
    json_ok(())
}

#[endpoint(tags("Menu"))]
pub async fn delete_menu(_token: CookieParam<String, true>, data: JsonBody<MenuId>) -> JsonResult<()> {
    let MenuId {id} = data.into_inner();
    let conn = db::conn().await?;
    let res = menu::Entity::delete_by_id(id).exec(&conn).await?;
    if res.rows_affected == 0 {
        return Err(AppError::Internal("删除失败".to_string()));
    }

    json_ok(())
}

#[endpoint(tags("Menu"))]
pub async fn get_menu(_token: CookieParam<String, true>) -> JsonResult<Vec<MenuModel>> {
    let conn = db::conn().await?;
    let res = menu::Entity::find().into_partial_model::<MenuModel>().all(&conn).await?;
    json_ok(res)
}