#![allow(non_snake_case)]
#![feature(log_syntax, proc_macro_quote)]
extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

use quote::{quote, ToTokens};
use syn::{ItemStruct};
use syn::__private::TokenStream2;

#[cfg(test)]
mod _test_utils;
#[doc(hidden)]
mod _models;
#[doc(hidden)]
mod _utils;
#[doc(hidden)]
mod service_derive;
#[doc(hidden)]
mod lazy_derive;

/// Service annotation
#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let source = TokenStream2::from(item.clone());
    let attribute = service_derive::ServiceAttr::parse_attr(attr.clone());

    if let Ok(construct_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let body = construct_fn.block.to_token_stream();
        return service_derive::impl_service_construct(construct_fn, &body);
    };

    if let Ok(service_struct) = syn::parse::<ItemStruct>(item.clone()) {
        let strict_name = service_struct.ident;
        let static_impl = service_derive::impl_static(&strict_name, &service_struct.vis);
        let into_impl = service_derive::impl_instance(&strict_name);
        let mut impl_service = TokenStream2::from_str("").unwrap();

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

/// Lazy annotation
#[proc_macro_attribute]
pub fn lazy(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(lazy_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let source = TokenStream2::from(item);
        let attribute = lazy_derive::LazyAttr::parse_attr(attr.clone());
        let static_impl = lazy_derive::impl_lazy_static(lazy_fn, attribute);

        let result = quote! {
            #source
            #[automatically_derived]
            #static_impl
        };

        return result.into();
    };

    item
}