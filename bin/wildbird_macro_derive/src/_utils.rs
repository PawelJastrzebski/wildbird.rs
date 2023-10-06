use proc_macro::TokenStream;
use quote::{quote_spanned, ToTokens};
use std::collections::HashMap;
use std::str::FromStr;
use syn::Visibility;
use syn::__private::TokenStream2;

pub fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

pub fn get_public_token(vis: &Visibility) -> TokenStream2 {
    match is_public(vis) {
        true => vis.to_token_stream(),
        false => TokenStream2::from_str("").unwrap(),
    }
}

pub fn parse_attr_to_map(attr: TokenStream) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for entry in attr.to_string().split(',') {
        let key_value = entry.split('=').collect::<Vec<&str>>();
        let key = key_value.get(0).unwrap_or(&"");
        let value = key_value.get(1).unwrap_or(&"");
        map.insert(
            key.trim().to_string(),
            value.trim().replace('\"', "").to_string(),
        );
    }
    map
}

pub fn error(span: syn::__private::Span, message: String) -> TokenStream2 {
    quote_spanned!( span => compile_error!(#message); )
}
