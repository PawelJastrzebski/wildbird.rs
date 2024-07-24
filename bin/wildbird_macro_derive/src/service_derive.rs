use crate::_utils::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    FnArg, Ident, ItemFn, ItemStruct, ReturnType, Visibility, __private::TokenStream2,
    spanned::Spanned,
};

struct ServiceAttr {
    pub construct: String,
}

impl ServiceAttr {
    pub fn parse_attr(attr: TokenStream) -> ServiceAttr {
        let map = parse_attr_to_map(attr);
        ServiceAttr {
            construct: map
                .get("construct")
                .or(map.get("constructor"))
                .unwrap_or(&"".to_string())
                .clone(),
        }
    }
}

#[inline]
fn _impl_static(struct_name: &Ident, visibility: &Visibility) -> TokenStream2 {
    let visibility_token = get_public_token(visibility);
    quote! {
        #[allow(non_upper_case_globals)]
         #visibility_token static #struct_name: wildbird::Lazy<#struct_name> = wildbird::private::service_construct::<#struct_name>();
    }
}

#[inline]
fn _impl_instance(struct_name: &Ident) -> TokenStream2 {
    quote! {
         impl #struct_name {
            fn instance(&self) -> std::sync::Arc<#struct_name> { #struct_name.instance() }
        }

        impl wildbird::private::PrivateService<#struct_name> for #struct_name {
            fn lazy() -> &'static wildbird::Lazy<#struct_name> { &#struct_name }
        }

        impl wildbird::private::PrivateService<#struct_name> for & #struct_name {
            fn lazy() -> &'static wildbird::Lazy<#struct_name> { &#struct_name }
        }

        impl wildbird::private::PrivateService<#struct_name> for std::sync::Arc<#struct_name> {
            fn lazy() -> &'static wildbird::Lazy<#struct_name> { &#struct_name }
        }

        impl wildbird::private::PrivateService<#struct_name> for wildbird::Lazy<#struct_name> {
            fn lazy() -> &'static wildbird::Lazy<#struct_name> { &#struct_name }
        }

        impl std::convert::From<&'static wildbird::Lazy<#struct_name>> for & #struct_name {
            fn from(value: &'static wildbird::Lazy<#struct_name>) -> Self {
                &value
            }
        }
    }
}

struct ConstructFn {
    fn_name: TokenStream2,
    is_async: bool,
    new_inner_fn: TokenStream2,
}

fn _construct_function_inject(fun: &ItemFn, errors: &mut CompileErrors) -> ConstructFn {
    let args: Vec<&FnArg> = fun.sig.inputs.iter().collect();
    let fn_name = &fun.sig.ident;
    let is_async = fun.sig.asyncness.is_some();

    if args.is_empty() {
        return ConstructFn {
            fn_name: fn_name.to_token_stream(),
            is_async,
            new_inner_fn: TokenStream2::default(),
        };
    }

    let new_fn_name = format_ident!("{}_inject", fn_name);
    let mut fn_args = quote!();
    let fn_return = &fun.sig.output;

    for arg in args {
        match arg {
            FnArg::Receiver(s) => {
                errors.add_spaned(s.span(), "'self' not allowed here".to_string());
                fn_args.append_all(quote!(wildbird::Inject()));
            }
            FnArg::Typed(_) => {
                if fn_args.is_empty() {
                    fn_args.append_all(quote!(wildbird::Inject()))
                } else {
                    fn_args.append_all(quote!(, wildbird::Inject()))
                }
            }
        }
    }

    let new_inner_fn = if is_async {
        quote!(
            async fn #new_fn_name() #fn_return {
                #fn_name (#fn_args).await
            }
        )
    } else {
        quote!(
            fn #new_fn_name() #fn_return {
                #fn_name (#fn_args)
            }
        )
    };

    return ConstructFn {
        fn_name: new_fn_name.to_token_stream(),
        new_inner_fn,
        is_async,
    };
}

fn _impl_service_construct_for_function(fun: ItemFn, errors: &mut CompileErrors) -> TokenStream2 {
    let ConstructFn {
        fn_name,
        is_async,
        new_inner_fn,
    } = _construct_function_inject(&fun, errors);
    match fun.sig.output {
        ReturnType::Default => {
            errors.add(format!("Specify function return type for: {}()", fn_name));
            return TokenStream2::default();
        }
        ReturnType::Type(_, t) => {
            let service_type = t.to_token_stream();
            let body = if is_async {
                quote! {
                    {
                        #new_inner_fn
                        wildbird::private::block(async { #fn_name().await })
                    }
                }
            } else {
                quote! {
                    {
                        #new_inner_fn
                        #fn_name()
                    }
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
    let mut errors = CompileErrors::default();

    if let Ok(construct_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let impl_service = _impl_service_construct_for_function(construct_fn, &mut errors);

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
        let mut impl_service = TokenStream2::default();

        if !attribute.construct.is_empty() {
            let is_async = attribute.construct.contains("async");
            let method_name = attribute.construct.replace("async", "").trim().to_string();
            let body = _impl_service_body(method_name, &strict_name, is_async);
            impl_service = _impl_service(&body, &strict_name.to_token_stream());
        }

        let res = quote!(
            #errors
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
