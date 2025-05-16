use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Path};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;
#[proc_macro_derive(InsertModel, attributes(sea_orm_ext))]
pub fn derive_insert_model(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        attrs,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let fields;

    if let Data::Struct(ds) = data {
        fields = ds.fields;
        if let Fields::Unit = fields {
            panic!("Struct must have fields");
        }
    } else {
        panic!("Only structs can be derived");
    }
    
    let mut mod_name: Option<Path> = None;
    let mut field_names: Vec<String> = Vec::new();

    let res = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sea_orm_ext"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("mod") {
                    mod_name = Some(meta.value()?.parse()?);
                }
                Ok(())
            })
        })
        .map_err(Error::into_compile_error);
    if let Err(et) = res {
        return et.into();
    }

    let (model_path, entity_path) = match mod_name {
        Some(mod_name) => (quote! {#mod_name::Model}, quote! {#mod_name::Entity}),
        None => (quote! {Model}, quote! {Entity}),
    };

    match fields  {
        Fields::Named(fields_named) => {
            fields_named.named.iter().for_each(|f| {
                field_names.push(f.ident.as_ref().unwrap().to_string());
            });
        },
        Fields::Unnamed(_) => panic!("Unnamed structs are not supported"),
        Fields::Unit => panic!("Unit structs are not supported"),
    }

    let column_names = field_names.join(",");
    let values_placeholders = field_names.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<String>>().join(",");
    let values = field_names
        .iter()
        .map(|f| format!("self.{}.clone().into()", f))
        .collect::<Vec<String>>()
        .join(",")
        .parse::<TokenStream2>()
        .unwrap();

    quote! {
        #[automatically_derived]
        impl sea_orm_ext::InsertModelTrait for #ident {
            type Model = #model_path;
            async fn insert(&self, conn: &DatabaseConnection) -> Result<Self::Model, sea_orm::DbErr> {
                let table_name = #entity_path.table_name();
                let sql = format!("INSERT INTO {} ({}) VALUES({}) RETURNING *;", table_name, #column_names, #values_placeholders);
                let model = #model_path::find_by_statement(Statement::from_sql_and_values(
                    conn.get_database_backend(),
                    sql,
                    [#values])).one(conn).await?;
                if let Some(model) = model {
                    Ok(model)
                } else {
                    Err(sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(format!("Insert `{}` Failed.", table_name))))
                }
            }
        }
    }.into()
}