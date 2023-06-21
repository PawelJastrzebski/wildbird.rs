use super::Service;
use crate::Lazy;
use crate::Callback;

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
pub fn block_callback<D, F>(future: fn(Callback<D>) -> F) -> D
    where
        D: Send + Sync + 'static,
        F: std::future::Future<Output=()> + 'static,
{
    let (tx, rx) = std::sync::mpsc::sync_channel(0);
    std::thread::spawn(move || {
        futures_lite::future::block_on(
            future(Callback::new(tx))
        );
    });
    rx.recv().unwrap()
}

#[inline]
#[doc(hidden)]
pub fn block<T>(future: impl std::future::Future<Output=T>) -> T {
    futures_lite::future::block_on(future)
}
