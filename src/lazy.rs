use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

#[doc(hidden)]
pub struct Lazy<T>(OnceLock<Arc<T>>, fn() -> T);

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self._get().as_ref()
    }
}

impl<T: Display> Display for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<T: Debug> Debug for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Lazy<T> {
        Lazy(OnceLock::new(), init)
    }

    fn _get(&self) -> &Arc<T> {
        self.0.get_or_init(|| Arc::new((self.1)()))
    }

    pub fn clone_lazy(&self) -> Self {
        Lazy(OnceLock::from(self.instance()), self.1)
    }

    pub fn instance(&self) -> Arc<T> {
        self._get().clone()
    }

    pub fn to_ref(&self) -> &T {
        &self._get().as_ref()
    }
}
