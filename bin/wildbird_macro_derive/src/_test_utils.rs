use syn::parse::Parse;
use syn::__private::TokenStream2;

#[allow(dead_code)]
#[cfg(test)]
pub fn parse_tokens<T>(q: TokenStream2) -> T
    where T: Parse
{
    let str = q.to_string();
    syn::parse_str(&str).expect("valid TokenString syn parse")
}

#[cfg(test)]
#[macro_export]
macro_rules! q {
        ($($tt:tt)*) => {{
            parse_tokens(quote!($($tt)*))
        }};
}