#![feature(log_syntax)]
extern crate proc_macro;

use proc_macro::TokenStream;

#[cfg(test)]
mod _test_utils;
#[doc(hidden)]
mod _utils;
#[doc(hidden)]
mod service_derive;
#[doc(hidden)]
mod var_derive;

/// Service annotation
#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    service_derive::main(attr, item)
}

/// Var annotation
#[proc_macro_attribute]
pub fn var(attr: TokenStream, item: TokenStream) -> TokenStream {
    var_derive::main(attr, item)
}

