use crate::inject::InjectStack;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::panic::Location;
use std::sync::{Arc, OnceLock};

#[doc(hidden)]
pub struct Lazy<T> {
    instance: OnceLock<Arc<T>>,
    init: fn() -> T,
    id: (&'static str, u32),
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self._get().as_ref()
    }
}

impl<T: Display> Display for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(inner) = self._get_opt() {
            return Display::fmt(inner, f);
        };
        Display::fmt("(Not initialized)", f)
    }
}

impl<T: Debug> Debug for Lazy<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(inner) = self._get_opt() {
            return Debug::fmt(inner, f);
        };
        Debug::fmt("(Not initialized) - use to_ref()", f)
    }
}

impl<T> Lazy<T> {
    #[track_caller]
    pub const fn new(init: fn() -> T) -> Lazy<T> {
        let caller = Location::caller();
        let file = caller.file();
        let line = caller.line();

        Self {
            instance: OnceLock::new(),
            init,
            id: (file, line),
        }
    }

    fn _get(&self) -> &Arc<T> {
        match self._get_opt() {
            Some(instance) => instance,
            None => {
                let id = self.id();
                if InjectStack::has_id(&id) {
                    panic!("{}", InjectStack::cilcuar_error(id))
                }
                InjectStack::push_id(id.clone());
                let instance = self.instance.get_or_init(|| Arc::new((self.init)()));
                InjectStack::remove(&id);
                instance
            }
        }
    }

    fn id(&self) -> String {
        format!("{}:{}", self.id.0, self.id.1)
    }

    fn _get_opt(&self) -> Option<&Arc<T>> {
        self.instance.get()
    }

    pub fn clone_lazy(&self) -> Self {
        let instance = self.instance();
        Self {
            instance: OnceLock::from(instance),
            init: self.init.clone(),
            id: self.id.clone(),
        }
    }

    pub fn instance(&self) -> Arc<T> {
        self._get().clone()
    }

    pub fn to_ref(&self) -> &T {
        self._get().as_ref()
    }
}
