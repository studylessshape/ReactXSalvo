use sea_orm::{ActiveModelTrait, DatabaseConnection, ModelTrait};

/// It is recommended to implement using `#[derive(Clone, InsertModel)]`. But also can implement manually.
///
/// ```rust
/// mod user {
///     use sea_orm::{entity::prelude::*, FromQueryResult, Statement};
///     use sea_orm_ext::InsertModelTrait;
///
///     #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
///     #[sea_orm(table_name = "user")]
///     pub struct Model {
///         #[sea_orm(primary_key)]
///         pub id: i32,
///         pub name: String,
///         #[sea_orm(default_value = 1)]
///         pub status: i32,
///     }
///
///     #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
///     pub enum Relation {}
///     impl ActiveModelBehavior for ActiveModel {}
///
///     #[derive(Debug, Clone)]
///     pub struct InsertModel {
///         pub id: i32,
///         pub name: String,
///     }
///
///     impl InsertModelTrait for InsertModel  {
///         type Model = Model;
///         async fn insert(&self, conn: &DatabaseConnection) -> Result<Self::Model, sea_orm::DbErr> {
///             let model = user::Model::find_by_statement(
///                 Statement::from_sql_and_values(
///                     conn.get_database_backend(),
///                     "INSERT INTO sys_user (id, name) VALUES($1,$2) REUTRNING *;",
///                     [self.id.into(), self.name.clone().into()]
///                 )
///             ).one(conn).await?;
///             if let Some(model) = model {
///                 Ok(model)
///             } else {
///                 Err(sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal("Insert User Failed.".to_string())))
///             }
///         }
///     }
/// }
/// ```
pub trait InsertModelTrait {
    type Model: ModelTrait;
    fn insert(
        &self,
        conn: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<Self::Model, sea_orm::DbErr>> + Send;
}

/// Custom insert model to database.
pub trait InsertActiveModelTrait: ActiveModelTrait {
    type Model: ModelTrait;
    fn insert_active(
        &self,
        conn: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<Self::Model, sea_orm::DbErr>> + Send;
}

pub use sea_orm_ext_macro::*;

#[cfg(test)]
mod test {
    mod user {
        use sea_orm::entity::prelude::*;
        use sea_orm_ext_macro::InsertActiveModel;

        use crate::{test::user, *};

        use crate as sea_orm_ext;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, InsertActiveModel)]
        #[sea_orm(table_name = "user")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub name: String,
            #[sea_orm(default = 1)]
            pub status: i8,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}

        #[derive(Debug, Clone, InsertModel)]
        #[sea_orm_ext(mod = user)]
        pub struct InsertModel {
            pub id: i32,
            pub name: String,
        }
    }
}
