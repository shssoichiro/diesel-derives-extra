#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![recursion_limit = "256"]

extern crate diesel;
extern crate diesel_derives_traits;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};
use syn::{parse_derive_input, DeriveInput, Ident};

lazy_static! {
    static ref IDENTS: Vec<Ident> = {
        let mut idents = Vec::new();
        #[cfg(feature="postgres")]
        { idents.push(Ident::new("::diesel::pg::PgConnection")); }
//        #[cfg(feature="mysql")]
//        { idents.push(Ident::new("::diesel::mysql::MysqlConnection")); }
//        #[cfg(feature="sqlite")]
//        { idents.push(Ident::new("::diesel::sqlite::SqliteConnection")); }
        #[cfg(feature="logger")]
        {
            #[cfg(feature="postgres")]
            { idents.push(Ident::new("::diesel_logger::LoggingConnection<::diesel::pg::PgConnection>")); }
//            #[cfg(feature="mysql")]
//            { idents.push(Ident::new("::diesel_logger::LoggingConnection<::diesel::mysql::MysqlConnection>")); }
//            #[cfg(feature="sqlite")]
//            { idents.push(Ident::new("::diesel_logger::LoggingConnection<::diesel::sqlite::SqliteConnection>")); }
        }
        idents
    };
}

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    expand_derive(&input, impl_model)
}

fn impl_model(item: &DeriveInput) -> Tokens {
    let name = &item.ident;

    IDENTS
        .iter()
        .map(|conn_impl| {
            quote!(
        impl<'a> ::diesel_derives_traits::Model<'a, #conn_impl> for #name
        {
            fn save(self, conn: &#conn_impl) -> ::diesel::result::QueryResult<Self> {
                ::diesel::RunQueryDsl::get_result(
                    ::diesel::update(
                        &self
                    )
                    .set(&self),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn find_all(conn: &#conn_impl) -> ::diesel::result::QueryResult<Vec<Self>> {
                ::diesel::RunQueryDsl::load(
                    <Self as ::diesel::associations::HasTable>::table(),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn find_one(conn: &#conn_impl, id: <&'a Self as ::diesel::Identifiable>::Id) -> ::diesel::result::QueryResult<Option<Self>> {
                use diesel::{OptionalExtension, QueryDsl};

                ::diesel::RunQueryDsl::get_result(
                    <Self as ::diesel::associations::HasTable>::table().find(id),
                    conn
                )
                .optional()
                .map_err(|e| e.into())
            }

            fn exists(conn: &#conn_impl, id: <&'a Self as ::diesel::associations::Identifiable>::Id) -> ::diesel::result::QueryResult<bool> {
                use diesel::QueryDsl;

                ::diesel::RunQueryDsl::get_result(
                    ::diesel::select(::diesel::dsl::exists(<Self as ::diesel::associations::HasTable>::table().find(id))),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn count_all(conn: &#conn_impl) -> ::diesel::result::QueryResult<i64> {
                use diesel::QueryDsl;

                ::diesel::RunQueryDsl::get_result(
                    <Self as ::diesel::associations::HasTable>::table().count(),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn destroy(self, conn: &#conn_impl) -> ::diesel::result::QueryResult<()> {
                ::diesel::RunQueryDsl::execute(
                    ::diesel::delete(&self),
                    conn
                )?;
                Ok(())
            }
        }
    )
        })
        .fold(Tokens::new(), |mut tokens, cur| {
            tokens.append(cur);
            tokens
        })
}

#[proc_macro_derive(NewModel, attributes(model))]
pub fn derive_new_model(input: TokenStream) -> TokenStream {
    expand_derive(&input, impl_new_model)
}

fn impl_new_model(item: &syn::DeriveInput) -> Tokens {
    let name = &item.ident;
    let mut tokens = Tokens::new();
    name.to_tokens(&mut tokens);
    item.generics.to_tokens(&mut tokens);
    let target = item.attrs
        .iter()
        .find(|attr| attr.name() == "model")
        .expect("\"model\" attribute must be specified for #[derive(NewModel)]");
    let target_name = match target.value {
        syn::MetaItem::List(_, ref options) if options.len() >= 1 => match options[0] {
            syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref name)) => name.clone(),
            _ => panic!("Must be in the form of `#[model(MyModel)]`"),
        },
        _ => panic!("Must be in the form of `#[model(MyModel)]`"),
    };

    IDENTS
        .iter()
        .map(|conn_impl| {
            quote!(
            impl<'a> ::diesel_derives_traits::NewModel<'a, #target_name, #conn_impl> for #tokens
            {
                fn save(self, conn: &#conn_impl) -> ::diesel::result::QueryResult<#target_name> {
                    ::diesel::RunQueryDsl::get_result(
                        ::diesel::insert_into(<#target_name as ::diesel::associations::HasTable>::table())
                            .values(&self),
                        conn
                    )
                    .map_err(|e| e.into())
                }
            }
        )
        })
        .fold(Tokens::new(), |mut tokens, cur| {
            tokens.append(cur);
            tokens
        })
}

fn expand_derive(input: &TokenStream, f: fn(&DeriveInput) -> Tokens) -> TokenStream {
    let item = parse_derive_input(&input.to_string()).unwrap();
    f(&item).to_string().parse().unwrap()
}
