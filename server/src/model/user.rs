use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub account: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub const USER_TABLE: &str = "public.users";
}

crud!(User{}, User::USER_TABLE);

impl_select!(User{select_by_account(account: &str) -> Option => "`where account = #{account}`"}, User::USER_TABLE);
impl_select!(User{select_by_id(id: &str) -> Option => "`where id = #{id}`"}, User::USER_TABLE);
