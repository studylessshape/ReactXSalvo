pub mod menu;
pub mod role;
pub mod user;

macro_rules! entity {
    ($(- $path: path, $name: ident)+) => {
        $(
            pub use $path::{Entity as $name};
        )+
    };
    ($(- $path: path, $name: ident, $model_name: ident)+) => {
        $(
            pub use $path::{Entity as $name, Model as $model_name};
        )+
    };
}

entity!(
    - user, User, UserModel
    - menu, Menu, MenuModel
    - role, Role, RoleModel
);

use anyhow::Result;

use crate::db::conn;

macro_rules! create_table {
    ($conn: expr, $($entity: expr),+) => {{
        use sea_orm::{ConnectionTrait, Schema};
        let builder = $conn.get_database_backend();
        let schema = Schema::new(builder);
        $(
            let stmt = builder.build(schema.create_table_from_entity($entity).if_not_exists());
            let _ = $conn.execute(stmt).await;
        )+
    }};
}

pub async fn init() -> Result<()> {
    let conn = conn().await?;
    create_table!(conn, User, Menu, Role);
    conn.close().await?;
    Ok(())
}
