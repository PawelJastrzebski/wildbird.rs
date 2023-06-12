#![doc = include_str!("../README.md")]

/// Macro System
pub extern crate wildbird_macro_derive as derive;

/// Service Trait
pub trait Service {
    type Service;
    fn construct() -> Self::Service;
}


/// Private Module
///
/// **Do not use in your code**
pub mod private {
    use super::Service;
    use std::sync::Arc;
    use once_cell::sync::Lazy;

    #[doc(hidden)]
    pub type ServiceLazy<T> = Lazy<Arc<T>>;

    #[doc(hidden)]
    pub const fn service_construct<S: Service>() -> ServiceLazy<S::Service> {
        Lazy::new(|| { Arc::new(S::construct()) })
    }
}

