use proc_macro::TokenStream;
use syn::__private::TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Visibility, ItemFn, ReturnType};
use crate::_utils::*;

#[derive(Debug)]
pub struct ServiceAttr {
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
pub fn impl_static(struct_name: &Ident, visibility: &Visibility) -> TokenStream2 {
    let visibility_token = get_public_token(visibility);
    quote! {
        #[allow(non_upper_case_globals)]
         #visibility_token static #struct_name: wildbird::private::ServiceLazy<#struct_name> = wildbird::private::service_construct::<#struct_name>();
    }
}

#[inline]
pub fn impl_instance(struct_name: &Ident) -> TokenStream2 {
    quote! {
         impl #struct_name {
            fn instance(&self) -> std::sync::Arc<#struct_name> {
                use std::ops::Deref;
                #struct_name.deref().clone()
            }
        }
    }
}

#[inline]
pub fn impl_service_construct(fun: ItemFn) -> TokenStream2 {
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
            impl_service(&body, &service_type)
        }
    }
}

#[inline]
pub fn impl_service_body(method_name: String, strict_name: &Ident, is_async: bool) -> TokenStream2 {
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
pub fn impl_service(body: &TokenStream2, service_type: &TokenStream2) -> TokenStream2 {
    quote! {
        impl wildbird::Service for #service_type {
            type Service = #service_type;
            fn construct() -> Self::Service #body
        }
    }
}
