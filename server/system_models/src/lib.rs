pub mod menu;
pub mod menu_element;
pub mod role;
pub mod role_menu;
pub mod role_menu_element;
pub mod role_user;
pub mod user;

use sea_orm::DatabaseConnection;

macro_rules! create_table {
    ($conn: expr, $($entity: expr),+) => {{
        use sea_orm::{ConnectionTrait, Schema};
        let builder = $conn.get_database_backend();
        let schema = Schema::new(builder);
        $(
            let stmt = builder.build(schema.create_table_from_entity($entity).if_not_exists());
            let res = $conn.execute(stmt).await;
            if let Err(e) = res {
                tracing::event!(tracing::Level::ERROR, "{}", e);
            }
        )+
    }};
}

pub async fn init(conn: &DatabaseConnection) {
    create_table!(
        conn,
        user::Entity,
        menu::Entity,
        role::Entity,
        menu_element::Entity,
        role_menu::Entity,
        role_menu_element::Entity,
        role_user::Entity
    );
}
