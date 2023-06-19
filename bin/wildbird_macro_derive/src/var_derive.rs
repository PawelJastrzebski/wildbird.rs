use proc_macro::TokenStream;
use syn::__private::TokenStream2;
use std::str::FromStr;
use quote::{format_ident, quote, ToTokens};
use syn::{ItemFn, ReturnType};
use crate::_utils::*;

pub struct VarAttr {
    pub name: Option<String>,
}

impl VarAttr {
    pub fn parse_attr(attr: TokenStream) -> VarAttr {
        let map = parse_attr_to_map(attr);
        VarAttr {
            name: map.get("name").map(|v| { v.clone() })
        }
    }
}

#[inline]
fn impl_static(
    function_name: TokenStream2,
    const_name: String,
    return_type: &TokenStream2,
    visibility_token: TokenStream2,
) -> TokenStream2 {
    let static_name = TokenStream2::from_str(&*const_name).expect("Const name");
    quote! {
        #[allow(non_upper_case_globals)]
        #visibility_token static #static_name: wildbird::Lazy<#return_type> = wildbird::private::lazy_construct(#function_name);
    }
}

fn _impl_var_static(fun: ItemFn, attribute: VarAttr) -> TokenStream2 {
    let is_async = fun.sig.asyncness.is_some();
    let function_name = fun.sig.ident.to_token_stream();
    let const_name = attribute.name.unwrap_or(function_name.to_string().to_uppercase());
    match fun.sig.output {
        ReturnType::Default => {
            panic!("Specify function return type for: {function_name}()")
        }
        ReturnType::Type(_, r_type) => {
            let return_type = r_type.to_token_stream();
            let visibility_token = get_public_token(&fun.vis);

            if is_async {
                let init_function_name = format_ident!("_{}_init", function_name.to_string());
                let static_impl = impl_static(init_function_name.to_token_stream(), const_name, &return_type, visibility_token);

                quote!(
                    fn #init_function_name() -> #return_type {
                        wildbird::private::block(async { #function_name().await })
                    }
                    #static_impl
                )
            } else {
                impl_static(function_name, const_name, &return_type, visibility_token)
            }
        }
    }
}

pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(lazy_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let source = TokenStream2::from(item);
        let attribute = VarAttr::parse_attr(attr.clone());
        let static_impl = _impl_var_static(lazy_fn, attribute);

        let res = quote! {
            #source
            #[automatically_derived]
            #static_impl
        };
        return res.into();
    };

    item
}

