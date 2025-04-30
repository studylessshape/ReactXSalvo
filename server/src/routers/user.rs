use std::io::Error;

use salvo::oapi::{extract::JsonBody, ToSchema};
use salvo::{jwt_auth, prelude::*};
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DerivePartialModel, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::error::AppError;
use crate::hoops::jwt::JwtClaims;
use crate::model::user;
use crate::model::User;
use crate::{db, json_ok, utils, JsonResult};

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct AddUserInData {
    pub account: String,
    pub password: String,
}

#[derive(Serialize, ToSchema, Default, Debug)]
pub struct UserOutData {
   pub id: String,
}

#[endpoint(tags("User"))]
pub async fn add_user(user: JsonBody<AddUserInData>) -> JsonResult<UserOutData> {
    let AddUserInData { account, password } = user.into_inner();
    let conn = crate::db::conn().await?;
    if User::find().filter(user::Column::Account.eq(&account)).select_only().column(user::Column::Id).one(&conn).await?.is_some() {
        return Err(AppError::Public("账户已存在".to_string()));
    }
    let id = Ulid::new();
    let new_user = user::ActiveModel {
        id: Set(id.to_string()),
        account: Set(account.clone()),
        username: Set(account),
        password: Set(utils::hash_password(&password)?),
        create_time: Set(chrono::Local::now().naive_local()),
    };
    let db_user = new_user.insert(&conn).await?;
    json_ok(UserOutData { id: db_user.id })
}

#[derive(Serialize, ToSchema, Default, Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "crate::model::user::Entity")]
pub struct UserInfo {
    pub id: String,
    pub username: String,
}

#[endpoint(tags("User"))]
pub async fn get_user_profile(depot: &mut Depot) -> JsonResult<UserInfo> {
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

    let query = User::find_by_id(&claims.uid).into_partial_model::<UserInfo>().one(&conn).await?;

    if let Some(user_info) = query {
        return json_ok(user_info);
    }

    Err(StatusError::not_found().brief("未能找到用户").into())
}