#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![recursion_limit = "128"]

extern crate diesel;
extern crate diesel_derives_traits;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_model(&ast);
    gen.parse().unwrap()
}

fn impl_model(item: &syn::DeriveInput) -> Tokens {
    let name = &item.ident;

    quote! (
        impl<'a> ::diesel_derives_traits::Model<'a> for #name
        {
            fn save(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<Self> {
                ::diesel::RunQueryDsl::get_result(
                    ::diesel::update(
                        &self
                    )
                    .set(&self),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn find_all(conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<Vec<Self>> {
                ::diesel::RunQueryDsl::load(
                    <Self as ::diesel::associations::HasTable>::table(),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn find_one(conn: &::diesel::PgConnection, id: <&'a Self as ::diesel::associations::Identifiable>::Id) -> ::diesel::result::QueryResult<Option<Self>> {
                ::diesel::RunQueryDsl::get_result(
                    <Self as ::diesel::associations::HasTable>::table().find(id),
                    conn
                )
                .optional()
                .map_err(|e| e.into())
            }

            fn count_all(conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<i64> {
                ::diesel::RunQueryDsl::get_result(
                    <Self as ::diesel::associations::HasTable>::table().count(),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn destroy(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<()> {
                ::diesel::RunQueryDsl::execute(
                    ::diesel::delete(&self),
                    conn
                )?;
                Ok(())
            }
        }
    )
}

#[proc_macro_derive(NewModel)]
pub fn derive_new_model(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_new_model(&ast);
    gen.parse().unwrap()
}

fn impl_new_model(item: &syn::DeriveInput) -> Tokens {
    let name = &item.ident;

    quote! (
        impl ::diesel_derives_traits::NewModel<T: ::diesel::associations::HasTable> for #name
        {
            fn save(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<T> {
                ::diesel::RunQueryDsl::get_result(
                    ::diesel::insert_into(<Self as ::diesel::associations::HasTable>::table())
                        .values(&self),
                    conn
                )
                .map_err(|e| e.into())
            }
        }
    )
}
