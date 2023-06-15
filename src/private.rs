use std::ops::Deref;
use super::Service;
use std::sync::{Arc, OnceLock};

#[doc(hidden)]
pub struct ServiceLazy<T, F = fn() -> Arc<T>> {
    lock: OnceLock<Arc<T>>,
    init_fn: F,
}

impl<T> Deref for ServiceLazy<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.lock.get_or_init(&self.init_fn)
    }
}

impl<T> ServiceLazy<T>
{
    pub const fn new(fun: fn() -> Arc<T>) -> ServiceLazy<T> {
        ServiceLazy {
            lock: OnceLock::new(),
            init_fn: fun,
        }
    }

    pub fn instance(&self) -> Arc<T> {
        self.lock.get_or_init(&self.init_fn).clone()
    }
}

#[doc(hidden)]
pub const fn service_construct<S: Service>() -> ServiceLazy<S::Service> {
    ServiceLazy::new(|| { Arc::new(S::construct()) })
}