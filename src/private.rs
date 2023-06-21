use std::future::Future;
use crate::{Service, Lazy, Callback};

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
pub fn block_callback<T, F, O>(future: fn(Callback<T>) -> F) -> T
    where
        T: Send + Sync + 'static,
        F: Future<Output=O> + 'static,
{
    let (tx, rx) = std::sync::mpsc::sync_channel(0);
    std::thread::spawn(move || {
        futures_lite::future::block_on(
            future(Callback::new(tx))
        );
    });
    rx.recv().expect("\nCallback<T>.call(T) was not called\n")
}

#[inline]
#[doc(hidden)]
pub fn block<T>(future: impl Future<Output=T>) -> T {
    futures_lite::future::block_on(future)
}

#[inline]
#[doc(hidden)]
pub fn block_fn<D, F>(future: fn() -> F) -> D
    where F: Future<Output=D>
{
    futures_lite::future::block_on(future())
}
