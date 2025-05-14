use cookie::Cookie;
use salvo::oapi::{extract::JsonBody, ToSchema};
use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::hoops::jwt;
use crate::model::user;
use crate::{config, json_ok, utils, JsonResult};

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct LoginData {
    pub account: String,
    pub password: String,
}

#[derive(Serialize, ToSchema, Default, Debug)]
pub struct LoginOutData {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}

#[endpoint(tags("Auth"))]
pub async fn post_login(data: JsonBody<LoginData>, res: &mut Response) -> JsonResult<LoginOutData> {
    let data = data.into_inner();
    let id; let username;
    if data.account == config::SYSTEM_ACCOUNT && data.password == config::SYSTEM_PASSWORD {
        id = config::SYSTEM_ACCOUNT.to_string();
        username = config::SYSTEM_ACCOUNT.to_string();
    }
    else {
        let conn = crate::db::conn().await?;
        let user = user::Entity::find().filter(user::Column::Account.eq(&data.account)).one(&conn).await?;
        if user.is_none() {
            return Err(StatusError::unauthorized().brief("未找到用户").into());
        }
        let user::Model {
            id: uid,
            username: uname,
            password,
            ..
        } = user.unwrap();
        utils::verify_password(&data.password, &password)?;
        id = uid.to_string();
        username = uname;
    }

    let (token, exp) = jwt::get_token(&id)?;
    let odata = LoginOutData {
        id,
        username,
        token,
        exp,
    };
    let cookie = Cookie::build(("jwt_token", odata.token.clone()))
        .path("/")
        .http_only(true)
        // .expires(OffsetDateTime::from_unix_timestamp(exp).map_err(|e| StatusError::internal_server_error().brief(format!("{e}")))?)
        .build();
    res.add_cookie(cookie);
    json_ok(odata)
}