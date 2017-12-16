#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![recursion_limit = "128"]

extern crate diesel;
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
        impl<'a> ::Model<'a> for #name
        {
            fn save(self, conn: &::diesel::PgConnection) -> ::errors::Result<Self> {
                ::diesel::RunQueryDsl::get_result(
                    ::diesel::update(
                        &self
                    )
                    .set(&self),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn find_all(conn: &::diesel::PgConnection) -> ::errors::Result<Vec<Self>> {
                ::diesel::RunQueryDsl::load(
                    <Self as ::diesel::associations::HasTable>::table(),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn find_one(conn: &::diesel::PgConnection, id: <Self as ::diesel::associations::Identifiable>::id()) -> ::errors::Result<Option<Self>> {
                ::diesel::RunQueryDsl::get_result(
                    <Self as ::diesel::associations::HasTable>::table().find(id),
                    conn
                )
                .optional()
                .map_err(|e| e.into())
            }

            fn exists(conn: &::diesel::PgConnection, id: <Self as ::diesel::associations::Identifiable>::id()) -> ::errors::Result<bool> {
                ::diesel::RunQueryDsl::get_result(
                    ::diesel::select(::diesel::exists(<Self as ::diesel::associations::HasTable>::table())),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn count_all(conn: &::diesel::PgConnection) -> ::errors::Result<i64> {
                ::diesel::RunQueryDsl::get_result(
                    <Self as ::diesel::associations::HasTable>::table().count(),
                    conn
                )
                .map_err(|e| e.into())
            }

            fn destroy(self, conn: &::diesel::PgConnection) -> ::errors::Result<()> {
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
        impl<'a> ::NewModel<'a> for #name
        {
            fn save<T: ::Model>(self, conn: &::diesel::PgConnection) -> ::errors::Result<T> {
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
