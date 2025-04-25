use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub account: String,
    pub username: String,
    pub password: String,
}

crud!(User{});

impl_select!(User{select_by_account(account: String) -> Option => "`where account = #{account}`"});