use std::{cell::RefCell, collections::VecDeque};

use crate::Lazy;

impl <R> From< &'static Lazy<R>> for std::sync::Arc<R> {
    fn from(value: &'static Lazy<R>) -> Self {
        value.instance()
    }
}

impl <T> From<&'static Lazy<T>> for Lazy<T> {
    fn from(value: &'static Lazy<T>) -> Self {
        value.clone_lazy()
    }
}

/// Service Injector
#[allow(non_snake_case)]
pub fn Inject<T, R>() -> R
where
    &'static Lazy<T>: Into<R> + 'static,
    R: crate::private::PrivateService<T> + 'static
{
    R::lazy().into()
}


// Cilcular dependency detection
thread_local! {
    static INJECT_STACK: RefCell<VecDeque<String>>  = const { RefCell::new(VecDeque::new()) };
}

pub struct InjectStack {}
impl InjectStack {
    
    pub fn push_id(id: String) {
        INJECT_STACK.with_borrow_mut(|v| v.push_back(id));
    }
    
    pub fn remove(id: &String) {
        INJECT_STACK.with_borrow_mut(|v| v.retain(|v| v != id));
    }
    
    pub fn has_id(id: &String) -> bool {
        INJECT_STACK.with_borrow(|v| v.contains(id))
    }

    pub fn cilcuar_error(id: String) -> String {
        let mut all = INJECT_STACK.take();
        all.push_back(id);
        let max_id = all.iter().map(|v| v.len()).max().unwrap_or(60);
        let max_id = max_id + 10;
        let line = "-".repeat(max_id / 2);

        let mut message = vec![format!("Circular dependency:\n|{line} < {line}|")];
        for id in all {
            let pad = " ".repeat(max_id - id.len());
            message.push(format!("| {id}{pad} |"));
        }
        message.push(format!("|{line} > {line}|\n"));
        message.join("\n")
    }
}
