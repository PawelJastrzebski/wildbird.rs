#![allow(non_snake_case, unused_imports)]
#![feature(log_syntax, proc_macro_quote)]
extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::str::FromStr;
use proc_macro2::Ident;

use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, ItemStruct, parse_macro_input, ReturnType, Visibility};
use syn::spanned::Spanned;

#[cfg(test)]
mod test_utils;
#[doc(hidden)]
mod models;
#[doc(hidden)]
mod utils;
#[doc(hidden)]
mod service_derive;

#[derive(Debug)]
struct ServiceAttr {
    pub construct: String,
    // pub private: bool,
}

impl ServiceAttr {
    fn parse_attr(attr: TokenStream) -> ServiceAttr {
        let map = utils::parse_attr_to_map(attr);

        ServiceAttr {
            // private: map.get("private").unwrap_or(&"false".to_string()).parse().unwrap_or(false),
            construct: map.get("construct").unwrap_or(&"".to_string()).clone(),
        }
    }
}

/// Service annotation
#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let source = proc_macro2::TokenStream::from(item.clone());
    let attribute = ServiceAttr::parse_attr(attr.clone());

    if let Ok(construct_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let body = construct_fn.block.to_token_stream();
        return service_derive::impl_service_construct(construct_fn, &body);
    };

    if let Ok(service_struct) = syn::parse::<syn::ItemStruct>(item.clone()) {
        let public = utils::is_public(&service_struct);
        let strict_name = service_struct.ident;
        let static_impl = service_derive::impl_static(&strict_name, !public);
        let into_impl = service_derive::impl_instance(&strict_name);
        let mut impl_service = proc_macro2::TokenStream::from_str("").unwrap();

        if !attribute.construct.is_empty() {
            let body = service_derive::impl_service_body(attribute.construct, &strict_name);
            impl_service = service_derive::impl_service(&body, &strict_name.to_token_stream());
        }

        let res = quote!(
            #source
            #[automatically_derived]
            #static_impl
            #impl_service
            #into_impl
        );

        return res.into();
    };

    item
}

#[cfg(test)]
mod parser_tests {
    use proc_macro::TokenStream;

    use quote::*;
    use syn::*;

    use super::models::parse_fields;
    use crate::test_utils::*;
    use crate::q;

    #[test]
    fn should_parse_fields() {
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