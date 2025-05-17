use std::io::Error;

use salvo::oapi::extract::*;
use salvo::oapi::{extract::JsonBody, ToSchema};
use salvo::{jwt_auth, prelude::*};
use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::sqlx::types::chrono;
use sea_orm::{ColumnTrait, DerivePartialModel, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use sea_orm_ext::InsertModelTrait;
use crate::error::AppError;
use crate::hoops::jwt::JwtClaims;
use system_models::user;
use crate::{db, json_ok, utils, JsonResult};

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct AddUserInData {
    pub account: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "system_models::user::Entity")]
pub struct UserId {
   pub id: Uuid,
}

#[endpoint(tags("User"))]
pub async fn add_user(user: JsonBody<AddUserInData>, _token: CookieParam<String, true>) -> JsonResult<UserId> {
    let AddUserInData { account, password } = user.into_inner();
    let conn = crate::db::conn().await?;
    if user::Entity::find().filter(user::Column::Account.eq(&account)).into_partial_model::<UserId>().one(&conn).await?.is_some() {
        return Err(AppError::Public("账户已存在".to_string()));
    }
    let id = Uuid::now_v7();
    let new_user = user::NewUser {
        id,
        account: account.clone(),
        username: account,
        password: utils::hash_password(&password)?,
        create_time: chrono::Local::now().naive_utc(),
    };
    let db_user = new_user.insert(&conn).await?;
    json_ok(UserId { id: db_user.id })
}

#[derive(Serialize, ToSchema, Default, Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "system_models::user::Entity")]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
}

#[endpoint(tags("User"))]
pub async fn get_user_profile(depot: &mut Depot, _token: CookieParam<String, true>) -> JsonResult<UserProfile> {
    let claims_res = depot.get::<JwtClaims>(jwt_auth::JWT_AUTH_DATA_KEY);
    let claims = claims_res.map_err(|e| if let Some(err_any) = e {
        if let Some(err) = err_any.downcast_ref::<Error>() {
            AppError::Public(err.to_string())
        } else {
            StatusError::internal_server_error().brief(format!("获取用户时发生未知错误：{:?}", err_any)).into()
        }
    } else {
        StatusError::unauthorized().brief("未登录").into()
    })?;
    let conn = db::conn().await?;

    let query = user::Entity::find_by_id(Uuid::parse_str(&claims.uid).unwrap()).into_partial_model::<UserProfile>().one(&conn).await?;

    if let Some(user_info) = query {
        return json_ok(user_info);
    }

    Err(StatusError::not_found().brief("未能找到用户").into())
}

#[derive(Debug, Deserialize, ToSchema)]
struct UserInfoQuerty {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub page: u64,
    pub page_size: u64,
}

impl Default for UserInfoQuerty  {
    fn default() -> Self {
        Self { id: Default::default(), username: Default::default(), page: 1, page_size: 20 }
    }
}

#[derive(Serialize, ToSchema, Default, Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "system_models::user::Entity")]
struct UserInfo {
    pub id: Uuid,
    pub account: String,
    pub username: String,
    pub status: i8,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Debug, ToSchema, Serialize, Default)]
struct GetUserOutData {
    pub total: u64,
    pub data: Vec<UserInfo>,
}

#[endpoint(tags("User"))]
pub async fn get_users(params: QueryParam<UserInfoQuerty>, _token: CookieParam<String, true>) -> JsonResult<GetUserOutData> {
    let conn = db::conn().await?;
    let mut query = user::Entity::find();
    if let Some(id) = &params.id {
        query = query.filter(user::Column::Id.eq(*id));
    }

    if let Some(username) = &params.username {
        query = query.filter(user::Column::Username.eq(username)); 
    }

    let total = query.clone().count(&conn).await?;
    let data = query
        .offset((params.page.saturating_sub(1)) * params.page_size)
        .limit(params.page_size).into_partial_model::<UserInfo>().all(&conn).await?;

    json_ok(GetUserOutData { total, data })
}

#[endpoint(tags("User"))]
pub async fn del_user(id: JsonBody<UserId>) -> JsonResult<()> {
    let id = id.into_inner().id;
    let conn = db::conn().await?;
    let del_res = user::Entity::delete_by_id(id).exec(&conn).await?;
    if del_res.rows_affected == 0 {
        return Err(AppError::Public("删除失败".to_string()));
    }
    json_ok(())
}