use proc_macro::TokenStream;
use std::str::FromStr;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Visibility, ItemFn, ReturnType, ItemStruct, __private::TokenStream2};
use crate::_utils::*;

struct ServiceAttr {
    pub construct: String,
}

impl ServiceAttr {
    pub fn parse_attr(attr: TokenStream) -> ServiceAttr {
        let map = parse_attr_to_map(attr);
        ServiceAttr {
            construct: map.get("construct").unwrap_or(&"".to_string()).clone(),
        }
    }
}

#[inline]
fn _impl_static(struct_name: &Ident, visibility: &Visibility) -> TokenStream2 {
    let visibility_token = get_public_token(visibility);
    quote! {
        #[allow(non_upper_case_globals)]
         #visibility_token static #struct_name: wildbird::private::ServiceLazy<#struct_name> = wildbird::private::service_construct::<#struct_name>();
    }
}

#[inline]
fn _impl_instance(struct_name: &Ident) -> TokenStream2 {
    quote! {
         impl #struct_name {
            fn instance(&self) -> std::sync::Arc<#struct_name> { #struct_name.instance() }
        }
    }
}

fn _impl_service_construct(fun: ItemFn) -> TokenStream2 {
    let is_async = fun.sig.asyncness.is_some();
    let function_name = fun.sig.ident.to_token_stream();
    match fun.sig.output {
        ReturnType::Default => {
            panic!("Specify function return type for: {}()", function_name.to_string())
        }
        ReturnType::Type(_, t) => {
            let service_type = t.to_token_stream();
            let body = if is_async {
                quote! {
                    { wildbird::private::block(async { #function_name().await }) }
                }
            } else {
                quote! {
                    { #function_name() }
                }
            };
            _impl_service(&body, &service_type)
        }
    }
}

#[inline]
fn _impl_service_body(method_name: String, strict_name: &Ident, is_async: bool) -> TokenStream2 {
    let construct_method_name = format_ident!("{}", method_name);
    if is_async {
        quote! {
            { wildbird::private::block(async { #strict_name::#construct_method_name().await }) }
        }
    } else {
        quote! {
            { #strict_name::#construct_method_name() }
        }
    }
}

#[inline]
pub fn _impl_service(body: &TokenStream2, service_type: &TokenStream2) -> TokenStream2 {
    quote! {
        impl wildbird::Service for #service_type {
            type Service = #service_type;
            fn construct() -> Self::Service #body
        }
    }
}

pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let source = TokenStream2::from(item.clone());
    let attribute = ServiceAttr::parse_attr(attr.clone());

    if let Ok(construct_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let impl_service = _impl_service_construct(construct_fn);

        let res = quote!(
            #source
            #[automatically_derived]
            #impl_service
        );
        return res.into();
    };

    if let Ok(service_struct) = syn::parse::<ItemStruct>(item.clone()) {
        let strict_name = service_struct.ident;
        let static_impl = _impl_static(&strict_name, &service_struct.vis);
        let into_impl = _impl_instance(&strict_name);
        let mut impl_service = TokenStream2::from_str("").unwrap();

        if !attribute.construct.is_empty() {
            let is_async = attribute.construct.contains("async");
            let method_name = attribute.construct.replace("async", "").trim().to_string();
            let body = _impl_service_body(method_name, &strict_name, is_async);
            impl_service = _impl_service(&body, &strict_name.to_token_stream());
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
