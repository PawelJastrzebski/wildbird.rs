use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
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

pub struct CompileErrors {
    count: i32,
    quote: TokenStream2,
}

impl Default for CompileErrors {
    fn default() -> Self {
        Self {
            count: 0,
            quote: Default::default(),
        }
    }
}

impl ToTokens for CompileErrors {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(self.quote.clone())
    }
}

impl From<CompileErrors> for TokenStream {
    fn from(value: CompileErrors) -> Self {
        value.quote.into()
    }
}

#[allow(dead_code)]
impl CompileErrors {
    pub fn add(&mut self, message: String) {
        let msg = format!("[wildbird]\n{message}\n");
        self.quote.append_all(quote!( compile_error!(#msg); ));
        self.count = self.count + 1;
    }
    pub fn add_spaned(&mut self, span: syn::__private::Span, message: String) {
        let msg = format!("[wildbird]\n{message}\n");
        self.quote
            .append_all(quote_spanned!( span => compile_error!(#msg); ));
        self.count = self.count + 1;
    }

    pub fn has_errors(&self) -> bool {
        self.count > 1
    }
}
