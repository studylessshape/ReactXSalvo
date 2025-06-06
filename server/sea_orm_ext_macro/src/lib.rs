mod insert_model;
mod insert_active_model;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::ToTokens;

/// Derives the `InsertModel` trait for the given struct.
/// 
/// ## Examples
/// 
/// Example entity definition:
/// 
/// ```rust
/// mod user {
///     use sea_orm::{entity::prelude::*, FromQueryResult, Statement};
///     use sea_orm_ext::InsertModel;
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
/// 
///     impl ActiveModelBehavior for ActiveModel {}
/// }
/// ```
/// 
/// Define insert model in sample mod scope:
/// 
/// ```
/// mod user {
///     // ...Model code
/// 
///     #[derive(Clone, InsertModel)]
///     pub struct NewUser {
///         pub id: i32,
///         pub name: String,
///     }
/// }
/// ```
/// 
/// Or define in other mod scope:
/// 
/// ```rust
/// mod other {
///     #[derive(Clone, InsertModel)]
///     #[sea_orm_ext(mod = path_to::user)]
///     pub struct NewUser {
///         pub id: i32,
///         pub name: String,
///     }
/// }
/// ```
/// 
/// ## Todo
/// 
/// - [ ] Support custom column name.
#[proc_macro_derive(InsertModel, attributes(sea_orm_ext))]
pub fn derive_insert_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder = insert_model::InsertModelBuilder::new(input);
    builder.into_token_stream().into()
}

/// Automatically derive the `InsertActiveModel` trait for the given struct.
/// 
/// `NotSet` fields will be ignored.
/// 
/// ## Todo
/// 
/// - [ ] Support must set fields which type is not `Option<T>`.
#[proc_macro_derive(InsertActiveModel, attributes(sea_orm))]
pub fn derive_insert_active_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    insert_active_model::expand_insert_active_model(input).into()
}