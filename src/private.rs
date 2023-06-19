use super::Service;
use crate::lazy::Lazy;

#[doc(hidden)]
pub type ServiceLazy<T> = Lazy<T>;

#[inline]
#[doc(hidden)]
pub const fn service_construct<S: Service>() -> ServiceLazy<S::Service> {
    ServiceLazy::new(|| S::construct())
}

#[inline]
#[doc(hidden)]
pub const fn lazy_construct<T>(value: fn() -> T) -> Lazy<T> {
    Lazy::new(value)
}

#[inline]
#[doc(hidden)]
pub fn block<T>(future: impl std::future::Future<Output = T>) -> T {
    futures_lite::future::block_on(future)
}