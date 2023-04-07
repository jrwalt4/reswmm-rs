extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{self, DeriveInput, Data, DataStruct};

#[proc_macro_derive(IntoReal)]
pub fn into_real(tokens: TokenStream) -> TokenStream {
    let DeriveInput{ident, data, ..} = syn::parse(tokens).unwrap();
    match data {
        Data::Struct(DataStruct{fields, ..}) => {
            if fields.len() != 1 {
                panic!("Cannot impl IntoReal, {} fields", fields.len());
            }
            let field = match &fields.iter().next().unwrap().ident {
                Some(ident) => quote!{#ident},
                None => quote!{0}
            };
            let into_real_impl = quote! {
                impl std::convert::From<&#ident> for f32 {
                    fn from(other: &#ident) -> f32 { other.#field as f32 }
                }
                impl std::convert::From<&#ident> for f64 {
                    fn from(other: &#ident) -> f64 { other.#field as f64 }
                }
            };
            return into_real_impl.into();
        },
        _ => panic!("Cannot impl IntoReal, not struct")
    }
}
