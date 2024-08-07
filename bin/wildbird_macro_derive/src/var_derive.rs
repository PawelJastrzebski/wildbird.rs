use crate::_utils::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::str::FromStr;
use syn::{spanned::Spanned, FnArg, ItemFn, PatType, ReturnType, __private::TokenStream2};

pub struct VarAttr {
    pub name: Option<String>,
}

impl VarAttr {
    pub fn parse_attr(attr: TokenStream) -> VarAttr {
        let map = parse_attr_to_map(attr);
        VarAttr {
            name: map.get("name").cloned(),
        }
    }
}

#[inline]
fn impl_static(
    function_name: &TokenStream2,
    const_name: &str,
    return_type: &TokenStream2,
    visibility_token: &TokenStream2,
) -> TokenStream2 {
    let static_name = format_ident!("{const_name}");
    quote! {
        #[allow(non_upper_case_globals)]
        #visibility_token static #static_name: wildbird::Lazy<#return_type> = wildbird::private::lazy_construct(#function_name);
    }
}

fn _get_var_fn_output_type(fun: &ItemFn) -> TokenStream2 {
    match &fun.sig.output {
        ReturnType::Default => quote!(()).to_token_stream(),
        ReturnType::Type(_, r_type) => r_type.to_token_stream(),
    }
}

fn _impl_var_static(
    fun: &ItemFn,
    first_arg: Option<&&PatType>,
    attribute: VarAttr,
) -> TokenStream2 {
    let is_async = fun.sig.asyncness.is_some();
    let function_name = fun.sig.ident.to_token_stream();
    let visibility_token = get_public_token(&fun.vis);
    let mut return_type = _get_var_fn_output_type(fun);
    let static_name = attribute
        .name
        .unwrap_or(function_name.to_string().to_uppercase());

    if !is_async {
        return impl_static(
            &function_name,
            &static_name,
            &return_type,
            &visibility_token,
        );
    }

    let init_function_name = format_ident!("_{}_init", function_name.to_string()).to_token_stream();
    let mut static_impl = impl_static(
        &init_function_name,
        &static_name,
        &return_type,
        &visibility_token,
    );
    let mut body = quote!( wildbird::private::block_fn(#function_name) );
    if let Some(_callback_arg) = first_arg {
        return_type = unwrap_callback_type(_callback_arg.ty.to_token_stream());
        static_impl = impl_static(
            &init_function_name,
            &static_name,
            &return_type,
            &visibility_token,
        );
        body = quote!( wildbird::private::block_callback(#function_name) );
    }

    quote!(
        fn #init_function_name() -> #return_type { #body }
        #static_impl
    )
}

fn unwrap_callback_type(callback_arg_type: TokenStream2) -> TokenStream2 {
    let str = callback_arg_type.to_string().trim().replace(' ', "");
    let from = str.find("Callback<").map(|v| v + 9).unwrap_or(0);
    let to = str.rfind('>').unwrap_or(str.len());
    TokenStream2::from_str(&str[from..to]).expect("Valid type")
}

fn _var_validate(lazy_fn: &ItemFn, fields: &Vec<&PatType>, errors: &mut CompileErrors) {
    if let Some(first_arg) = fields.get(0) {
        
        let first_arg_type = first_arg
            .ty
            .to_token_stream()
            .to_string()
            .trim()
            .replace(' ', "");
        let first_arg_name = first_arg.pat.to_token_stream().to_string();

        if fields.len() == 1 && !first_arg_type.contains("Callback<") {
            errors.add_spaned(
                fields.as_slice()[0].ty.span(),
                "#[var] - Invalid \"callback\" first arg type: \n\texpected: Callback<T>"
                    .to_string(),
            );
        }

        if lazy_fn.sig.asyncness.is_none() {
            errors.add_spaned(
                lazy_fn.sig.span(),
                "#[var] - Callback function must by async".to_string(),
            );
        }

        let call_exist = format!("{first_arg_name}.call(");
        let body = lazy_fn
            .block
            .to_token_stream()
            .to_string()
            .trim()
            .replace(' ', "");
        if !body.contains(&call_exist) {
            errors.add_spaned(
                lazy_fn.block.span(),
                format!("#[var] - {first_arg_name}.call(T): method must be called"),
            );
        }

        if fields.len() > 1 {
            errors.add_spaned(
                lazy_fn.sig.inputs.span(),
                "#[var] - Invalid number of arguments\n\t zero or one (callback) allowed".to_string(),
            );
        }

    } else {
        if let ReturnType::Default = lazy_fn.sig.output {
            errors.add_spaned(
                lazy_fn.sig.span(),
                "Specify function return type".to_string(),
            );
        }
    };
}

pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(lazy_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let mut errors = CompileErrors::default();
        let source = TokenStream2::from(item.clone());
        let fields = parse_fn_args(&lazy_fn);

        // Validate signature
        _var_validate(&lazy_fn, &fields, &mut errors);
        if errors.has_errors() {
            return errors.into();
        }

        let first_arg = fields.get(0);
        let attribute = VarAttr::parse_attr(attr.clone());
        let static_impl = _impl_var_static(&lazy_fn, first_arg, attribute);

        let res = quote! {
            #errors
            #source
            #[automatically_derived]
            #static_impl
        };
        return res.into();
    };

    item
}

fn parse_fn_args(lazy_fn: &ItemFn) -> Vec<&PatType> {
    let mut fields = vec![];
    for t in lazy_fn.sig.inputs.iter() {
        if let FnArg::Typed(t) = t {
            fields.push(t)
        }
    }
    fields
}
