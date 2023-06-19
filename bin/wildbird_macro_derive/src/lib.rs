use proc_macro::TokenStream;
use std::str::FromStr;

use quote::{quote, ToTokens};
use syn::{ItemStruct};
use syn::__private::TokenStream2;

#[cfg(test)]
mod _test_utils;
#[doc(hidden)]
mod _utils;
#[doc(hidden)]
mod service_derive;
#[doc(hidden)]
mod var_derive;

/// Service annotation
#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let source = TokenStream2::from(item.clone());
    let attribute = service_derive::ServiceAttr::parse_attr(attr.clone());

    if let Ok(construct_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let impl_service = service_derive::impl_service_construct(construct_fn);

        let res = quote!(
            #source
            #[automatically_derived]
            #impl_service
        );
        return res.into();
    };

    if let Ok(service_struct) = syn::parse::<ItemStruct>(item.clone()) {
        let strict_name = service_struct.ident;
        let static_impl = service_derive::impl_static(&strict_name, &service_struct.vis);
        let into_impl = service_derive::impl_instance(&strict_name);
        let mut impl_service = TokenStream2::from_str("").unwrap();

        if !attribute.construct.is_empty() {
            let is_async = attribute.construct.contains("async");
            let method_name = attribute.construct.replace("async", "").trim().to_string();
            let body = service_derive::impl_service_body(method_name, &strict_name, is_async);
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

/// Var annotation
#[proc_macro_attribute]
pub fn var(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(lazy_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let source = TokenStream2::from(item);
        let attribute = var_derive::VarAttr::parse_attr(attr.clone());
        let static_impl = var_derive::impl_var_static(lazy_fn, attribute);

        let res = quote! {
            #source
            #[automatically_derived]
            #static_impl
        };
        return res.into();
    };

    item
}