use std::sync::mpsc::SyncSender;

pub struct Callback<T> {
    tx: SyncSender<T>,
}

impl<T> Callback<T> {
    pub fn new(tx: SyncSender<T>) -> Callback<T> {
        Callback { tx }
    }

    pub fn call(&self, init: T) -> () {
        let _ = self.tx.send(init);
    }
}