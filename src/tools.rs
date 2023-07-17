pub mod block {

    ///  Blocks async fn in non async environment
    /// > ⚠️ Calling from the main asynchronous thread will freeze the application
    ///
    /// # Example
    /// ```
    /// use wildbird::prelude::*;
    ///
    /// async fn fetch_from_api() -> String {
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    ///     String::from("Api respone")
    /// }
    ///
    /// let res = fetch_from_api().block();
    /// ```
    pub trait Block<F>
    where
        F: std::future::IntoFuture,
    {
        fn block(self) -> F::Output;
    }

    impl<F> Block<F> for F
    where
        F: std::future::IntoFuture,
    {
        fn block(self) -> F::Output {
            crate::private::block(self.into_future())
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::prelude::*;
        use std::time::Duration;

        async fn fetch_from_api() -> String {
            std::thread::sleep(Duration::from_millis(100));
            String::from("Api respone")
        }

        #[test]
        fn should_block() {
            println!("Res: {}", fetch_from_api().block())
        }

        #[tokio::test]
        async fn should_block_async() {
            println!("Res: {}", fetch_from_api().block())
        }

        #[test]
        fn should_block_async_block() {
            let action = async {
                let data1 = fetch_from_api().await;
                let data2 = fetch_from_api().await;
                format!("{data1}, {data2}")
            };

            println!("Data: {}", action.block())
        }
    }
}

pub mod lock {
    use std::sync::Mutex;

    /// Lock Mutex unsafe
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
}

pub mod error {

    /// Transform Result<> err to string
    pub trait ErrorToString<T> {
        fn map_err_str(self) -> Result<T, String>;
    }
    
    impl<T, E: std::error::Error> ErrorToString<T> for Result<T, E> {
        fn map_err_str(self) -> Result<T, String> {
            self.map_err(|e| e.to_string())
        }
    }
    
}