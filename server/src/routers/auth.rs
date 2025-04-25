use cookie::Cookie;
use salvo::oapi::{extract::JsonBody, ToSchema};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hoops::jwt;
use crate::model::user::User;
use crate::{json_ok, utils, JsonResult};

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
    let conn = crate::db::conn()?;
    let user = User::select_by_account(&conn, data.account).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("未找到用户").into());
    }
    let User {
        id,
        account: _,
        username,
        password,
    } = user.unwrap();
    utils::verify_password(&data.password, &password)?;

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
        .build();
    res.add_cookie(cookie);
    json_ok(odata)
}
