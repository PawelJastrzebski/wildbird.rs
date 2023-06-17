use super::Service;
use crate::lazy::Lazy;

#[doc(hidden)]
pub type ServiceLazy<T> = Lazy<T>;

#[doc(hidden)]
pub const fn service_construct<S: Service>() -> ServiceLazy<S::Service> {
    ServiceLazy::new(|| S::construct())
}

#[doc(hidden)]
pub const fn lazy_construct<T>(value: fn() -> T) -> Lazy<T> {
    Lazy::new(value)
}