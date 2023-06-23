use syn::__private::TokenStream2;
use syn::parse::Parse;

#[allow(dead_code)]
#[cfg(test)]
pub fn parse_tokens<T: Parse>(q: TokenStream2) -> T {
    let str = q.to_string();
    syn::parse_str(&str).expect("valid TokenString syn parse")
}

#[cfg(test)]
#[macro_export]
macro_rules! q {
        ($($tt:tt)*) => {{
            crate::_test_utils::parse_tokens(quote!($($tt)*))
        }};
}
