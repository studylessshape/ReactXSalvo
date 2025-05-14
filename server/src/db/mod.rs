use anyhow::{Ok, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::config;

pub async fn conn() -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(config::get().db.url.clone());
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;
    Ok(db)
}

pub async fn init_db() -> Result<()> {
    let conn = conn().await?;
    system_models::init(&conn).await;
    conn.close().await?;
    Ok(())
}