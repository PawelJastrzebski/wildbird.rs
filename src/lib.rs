pub extern crate wildbird_macro_derive as derive;

pub trait Service {
    type Service;
    fn construct() -> Self::Service;
}


/// Private Mod
pub mod private {
    use super::Service;
    use std::sync::Arc;
    use once_cell::sync::Lazy;

    pub type ServiceLazy<T> = Lazy<Arc<T>>;

    pub const fn service_construct<S: Service>() -> ServiceLazy<S::Service> {
        Lazy::new(|| { Arc::new(S::construct()) })
    }
}

