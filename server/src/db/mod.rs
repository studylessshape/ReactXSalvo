use anyhow::{Ok, Result};
use rbatis::{table_sync::PGTableMapper, RBatis};
use crate::{config, model::user::User};

pub fn conn() -> Result<RBatis> {
    let rb = RBatis::new();
    rb.init(rbdc_pg::Driver{}, &config::get().db.url)?;
    Ok(rb)
}

pub async fn init() -> Result<()> {
    let rb = conn()?;
    let conn = rb.acquire().await?;
    let user = User {
        id: "".to_string(),
        account: "".to_string(),
        username: "".to_string(),
        password: "".to_string(),
    };
    let _ = RBatis::sync(&conn, &PGTableMapper{}, &user, User::USER_TABLE).await;
    Ok(())
}
