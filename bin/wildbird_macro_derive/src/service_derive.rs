use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Data, Ident, parse_quote};
use crate::models::parse_fields;


pub fn impl_service_derive(ast: DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    // println!("derive(Service) for {}", struct_name.to_string());

    if let Data::Struct(data) = ast.data {
        let _fields = parse_fields(&data.fields);
    }

    let static_impl = impl_static(struct_name);
    let into_impl = impl_instance(struct_name);
    let res = quote!(
        #[automatically_derived]
        #static_impl
        #into_impl
    );

    res.into()
}


fn impl_static(struct_name: &Ident) ->  proc_macro2::TokenStream {
    quote! {
        #[allow(non_upper_case_globals)]
        static #struct_name: wildbird::private::ServiceLazy<#struct_name> = wildbird::private::service_construct::<#struct_name>();
    }
}

fn impl_instance(struct_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
         impl #struct_name {
            fn instance(&self) -> std::sync::Arc<#struct_name> {
                #struct_name.clone()
            }
        }
    }
}

use syn::{ItemFn, ReturnType};

pub fn impl_service_construct(fun: ItemFn) -> TokenStream {
    match fun.sig.output {
        ReturnType::Default => {
            let function_name = fun.sig.ident.to_token_stream().to_string();
            panic!("Specify function retrun type for: {function_name}()")
        }
        ReturnType::Type(_, t) => {
            let service_type = t.to_token_stream();
            let body = fun.block.to_token_stream();

            let gen = quote! {
                    impl wildbird::Service for #service_type {
                        type Service = #service_type;
                        fn construct() -> Self::Service #body
                    }
                };

            // println!("\
            //         Service: {service_type} \
            //         body: {body}\
            //         res: {}
            //     ", gen.to_token_stream().to_string());

            gen.into()
        }
    }
}
