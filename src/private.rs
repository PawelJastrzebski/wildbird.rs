use crate::{Callback, Lazy, Service};
use std::future::Future;

#[cfg(not(feature = "tokio"))]
use std::thread::spawn as spawn_blocking;
#[cfg(feature = "tokio")]
use tokio::task::spawn_blocking;

#[inline]
#[doc(hidden)]
pub const fn service_construct<S: Service>() -> Lazy<S::Service> {
    Lazy::new(|| S::construct())
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
    T: Send + 'static,
    F: Future<Output = O> + 'static,
{
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    let callback = Callback::new(tx);
    spawn_blocking(move || {
        block(future(callback));
    });
    rx.recv().expect("\nCallback<T>.call(T) was not called\n")
}

#[doc(hidden)]
#[cfg(feature = "tokio")]
static BLOCK_RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .thread_name("tokio block runtime")
        .build()
        .expect("tokio block runtime init")
});

#[inline(always)]
#[doc(hidden)]
// Panic in non multi-thread tokio runtime
pub fn block<T>(future: impl Future<Output = T>) -> T {
    #[cfg(feature = "tokio")]
    {
        tokio::task::block_in_place(move || BLOCK_RUNTIME.handle().block_on(future))
    }
    #[cfg(not(feature = "tokio"))]
    {
        futures_lite::future::block_on(future)
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn block_fn<D, F: Future<Output = D>>(future: fn() -> F) -> D {
    block(future())
}
