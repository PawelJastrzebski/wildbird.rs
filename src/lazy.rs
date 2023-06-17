use std::fmt::{self, Debug, Display, Formatter};
use std::sync::{Arc, OnceLock};
use std::ops::Deref;

#[doc(hidden)]
pub struct Lazy<T> {
    lock: OnceLock<Arc<T>>,
    init_fn: fn() -> T,
}

impl<T> Deref for Lazy<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.lock.get_or_init(|| Arc::new((self.init_fn)()))
    }
}

impl<T> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Lazy {
            init_fn: self.init_fn,
            lock: OnceLock::from(self.instance()),
        }
    }
}

impl<T: Display> Display for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.instance().deref().fmt(f)
    }
}

impl<T: Debug> Debug for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.instance().deref().fmt(f)
    }
}

impl<T> Lazy<T>
{
    pub const fn new(fun: fn() -> T) -> Lazy<T> {
        Lazy {
            lock: OnceLock::new(),
            init_fn: fun,
        }
    }

    pub fn instance(&self) -> Arc<T> {
        self.lock.get_or_init(|| Arc::new((self.init_fn)())).clone()
    }
}