use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, *};

struct NeedPath {
    model_path: TokenStream,
    entity_path: TokenStream,
}

impl TryFrom<&Vec<Attribute>> for NeedPath  {
    type Error = Error;

    fn try_from(value: &Vec<Attribute>) -> std::result::Result<Self, Self::Error> {
        let mut mod_path: Option<Path> = None;
        value
            .iter()
            .filter(|attr| attr.path().is_ident("sea_orm_ext"))
            .try_for_each(|attr| {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("mod") {
                        mod_path = Some(meta.value()?.parse()?);
                    }
                    Ok(())
                })
            })?;
        Ok(match mod_path {
            Some(p) => Self { model_path: quote!{#p::Model},entity_path: quote!{#p::Entity}},
            None => Self { model_path: quote!{Model},entity_path: quote!{Entity}}
        })
    }
}

pub struct InsertModelBuilder {
    ident: Ident,
    data: Data,
    attrs: Vec<Attribute>,
}

impl InsertModelBuilder {
    pub fn new(input: DeriveInput) -> Self {
        let DeriveInput { ident, data, attrs, .. } = input;
        Self { ident, data, attrs }
    }

    pub fn build(&self) -> Result<TokenStream> {
        let Self { ident, data, attrs } = self;
        let NeedPath { model_path, entity_path } = NeedPath::try_from(attrs)?;

        let mut columns= Vec::new();
        match data {
            Data::Struct(data_struct) => match &data_struct.fields {
                Fields::Named(fields_named) => fields_named.named.iter().for_each(|f| {
                    columns.push(f.ident.as_ref().unwrap().to_string());
                }),
                Fields::Unnamed(_) | Fields::Unit => return Err(Error::new(data_struct.struct_token.span(), "Don't support unnamed fields or unit struct")),
            },
            Data::Enum(data_enum) => return Err(Error::new(data_enum.enum_token.span(), "Enum not supported")),
            Data::Union(data_union) => return Err(Error::new(data_union.union_token.span(), "Union not supported")),
        }

        let column_names = columns.join(", ");
        let values_placeholders = columns.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<String>>().join(", ");
        let values = columns.iter().map(|c| format!("self.{}.clone().into()", c)).collect::<Vec<String>>().join(",").parse::<TokenStream>()?;

        
        Ok(quote! {
            #[automatically_derived]
            impl sea_orm_ext::InsertModelTrait for #ident {
                type Model = #model_path;
                async fn insert(&self, conn: &sea_orm::DatabaseConnection) -> Result<Self::Model, sea_orm::DbErr> {
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
        })
    }
}

impl ToTokens for InsertModelBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.build().unwrap_or_else(Error::into_compile_error));
    } 
}