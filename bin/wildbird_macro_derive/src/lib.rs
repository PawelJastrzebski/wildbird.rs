#![allow(non_snake_case, unused_imports)]
#![feature(log_syntax, proc_macro_quote)]
extern crate proc_macro;
extern crate core;

#[doc(hidden)]
mod models;

mod service_derive;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, ReturnType};

#[proc_macro_derive(Service)]
pub fn service_marcro_derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    service_derive::impl_service_derive(parsed)
}

#[proc_macro_attribute]
pub fn ServiceConstruct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let Ok(parsed) = syn::parse::<syn::ItemFn>(item.clone()) else {
        return item;
    };

    service_derive::impl_service_construct(parsed)
}

#[cfg(test)]
mod parser_tests {
    use proc_macro::TokenStream;
    use quote::*;
    use syn::*;
    use super::models::parse_fields;
    use super::test_utils::*;


    #[test]
    fn it_works1() {
        use syn::Type;
        let t: Type = syn::parse_str("std::collections::HashMap<String, Value>").unwrap();
    }

    #[test]
    fn it_works() {
        let r: ItemStruct = q!(
                struct Test {
                    name: String,
                    opt: Option<u32>
                }
        );

        println!("{:?}", &r.to_token_stream());
        let parsed = DeriveInput::from(r);

        let Data::Struct(data) = parsed.data else { panic!("invalid type") };
        let fields = parse_fields(&data.fields);
        println!("{:#?}", fields)
    }
}