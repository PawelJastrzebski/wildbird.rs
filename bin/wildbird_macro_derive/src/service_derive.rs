use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::str::FromStr;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Data, Ident, parse_quote};
use crate::models::parse_fields;

pub fn impl_static(struct_name: &Ident, private: bool) -> TokenStream2 {
    let visibility = if private { "" } else { "pub" };
    let visibility_token = TokenStream2::from_str(visibility).expect("visibility token");

    quote! {
        #[allow(non_upper_case_globals)]
         #visibility_token static #struct_name: wildbird::private::ServiceLazy<#struct_name> = wildbird::private::service_construct::<#struct_name>();
    }
}

pub fn impl_instance(struct_name: &Ident) -> TokenStream2 {
    quote! {
         impl #struct_name {
            fn instance(&self) -> std::sync::Arc<#struct_name> {
                #struct_name.clone()
            }
        }
    }
}

use syn::{ItemFn, ReturnType};

pub fn impl_service_construct(fun: ItemFn, body: &TokenStream2) -> TokenStream {
    match fun.sig.output {
        ReturnType::Default => {
            let function_name = fun.sig.ident.to_token_stream().to_string();
            panic!("Specify function retrun type for: {function_name}()")
        }
        ReturnType::Type(_, t) => {
            let service_type = t.to_token_stream();
            let gen = impl_service(body, &service_type);

            // println!("\
            //         Service: {service_type} \
            //         body: {body}\
            //         res: {}
            //     ", gen.to_token_stream().to_string());

            gen.into()
        }
    }
}

pub fn impl_service_body(method_name: String, strict_name: &Ident) -> TokenStream2 {
    let construct_method_name = format_ident!("{}", method_name);
    quote!(
        {
            #strict_name::#construct_method_name()
        }
    )
}

pub fn impl_service(body: &TokenStream2, service_type: &TokenStream2) -> TokenStream2 {
    quote! {
        impl wildbird::Service for #service_type {
            type Service = #service_type;
            fn construct() -> Self::Service #body
        }
    }
}
