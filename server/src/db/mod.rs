use anyhow::Result;
use rbatis::RBatis;
use rbdc_pg::Driver;

pub fn conn() -> Result<RBatis> {
    let rb = RBatis::new();
    rb.init(Driver{}, "postgresql://localhost:5432/postgres")?;
    Ok(rb)
}