use std::io::Error;

use salvo::oapi::{extract::JsonBody, ToSchema};
use salvo::{jwt_auth, prelude::*};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::error::AppError;
use crate::hoops::jwt::JwtClaims;
use crate::model::user::User;
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
    let conn = crate::db::conn()?;
    if User::select_by_account(&conn, &account).await?.is_some() {
        return Err(AppError::Public("账户已存在".to_string()));
    }
    let id = Ulid::new();
    User::insert(&conn, &User { id: id.to_string(), account: account.clone(), username: account, password: utils::hash_password(&password)? }).await?;
    json_ok(UserOutData { id: id.to_string() })
}

#[derive(Serialize, ToSchema, Default, Debug)]
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
    let conn = db::conn()?;
    if let Some(user) = User::select_by_id(&conn, &claims.uid).await? {
        return json_ok(UserInfo { id: user.id, username: user.username });
    }

    Err(StatusError::not_found().brief("未能找到用户").into())
}