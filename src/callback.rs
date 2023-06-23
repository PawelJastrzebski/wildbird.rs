use std::sync::mpsc::SyncSender;

// Callback for lazy initialization
#[doc(hidden)]
pub struct Callback<T>(SyncSender<T>);

impl<T> Callback<T> {
    pub fn new(tx: SyncSender<T>) -> Callback<T> {
        Callback(tx)
    }

    pub fn call(&self, init: T) -> () {
        let _ = self.0.send(init);
    }
}