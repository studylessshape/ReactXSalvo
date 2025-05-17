use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, *};
use heck::*;

fn build_derive(data: Data) -> Result<TokenStream> {
    let target_struct = (match data {
        Data::Struct(data_struct) => Ok(data_struct),
        Data::Enum(data_enum) => Err(Error::new(data_enum.enum_token.span(), "Enum not supported")),
        Data::Union(data_union) => Err(Error::new(data_union.union_token.span(), "Union not supported")),
    })?;

    let mut push_columns = TokenStream::new();

    (match target_struct.fields  {
        Fields::Named(fields_named) => {
            for field in fields_named.named {
                let field = field.ident.clone().unwrap();

                let column_ident = Ident::new(&field.to_string().to_upper_camel_case(), Span::call_site());
                push_columns.extend(quote! {
                    if let Some(#field) = self.#field.clone().into_value() {
                        columns.push(Column::#column_ident.as_str());
                        values.push(#field);
                    }
                });
            }
            Ok(())
        },
        Fields::Unnamed(fields_unnamed) => Err(Error::new(fields_unnamed.unnamed.span(), "Unnamed fields not supported")),
        Fields::Unit => Err(Error::new(target_struct.fields.span(), "Unit fields not supported")),
    })?;
    
    Ok(quote! {
        #[automatically_derived]
        impl sea_orm_ext::InsertActiveModelTrait for ActiveModel {
            type Model = Model;

            async fn insert_active(&self, conn: &sea_orm::DatabaseConnection) -> std::result::Result<Self::Model, sea_orm::DbErr> {
                use sea_orm::entity::prelude::*;
                use sea_orm::{Statement, FromQueryResult};
                
                let mut columns = Vec::new();
                let mut values = Vec::new();

                #push_columns

                if values.is_empty() {
                    return Err(sea_orm::DbErr::Custom("No column to insert".to_string()));
                }

                let table_name = Entity.table_name();
                let values_holder = (0..values.len()).map(|i| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
                let model = Model::find_by_statement(
                    Statement::from_sql_and_values(
                        conn.get_database_backend(),
                        format!("INSERT INTO {} ({}) VALUES ({}) RETURNING *;", table_name, columns.join(","), values_holder),
                        values,
                    )
                ).one(conn).await?;
                match model {
                    Some(model) => Ok(model),
                    None => Err(sea_orm::DbErr::Custom(format!("Insert `{}` failed.", table_name))),
                }
            }
        }
    })
}

pub fn expand_insert_active_model(input: DeriveInput) -> TokenStream {
    let DeriveInput { data, .. } = input;

    build_derive(data).unwrap_or_else(|err| err.to_compile_error())
}
