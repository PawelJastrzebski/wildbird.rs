use std::sync::Mutex;

pub trait LockUnsafe<'a, T> {
    fn lock_unsafe(&'a self) -> std::sync::MutexGuard<'a, T>;
}

impl<'a, T> LockUnsafe<'a, T> for Mutex<T> {
    fn lock_unsafe(&'a self) -> std::sync::MutexGuard<'a, T> {
        self.lock().expect("lock_unsafe failed")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Mutex;

    #[test]
    pub fn should_update_mutex() {
        let mx = Mutex::from("init".to_string());
        *mx.lock_unsafe() = "ok".to_string();

        let result = mx.lock().unwrap();
        assert_eq!("ok", result.trim())
    }
}