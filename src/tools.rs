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

    impl<T, E: ToString> ErrorToString<T> for Result<T, E> {
        fn map_err_str(self) -> Result<T, String> {
            self.map_err(|e| e.to_string())
        }
    }

    pub trait ErrorInto<T, E> {
        fn err_into(self) -> Result<T, E>;
    }

    impl<EI, T, E: Into<EI>> ErrorInto<T, EI> for Result<T, E> {
        fn err_into(self) -> Result<T, EI> {
            match self {
                Ok(ok) => Ok(ok),
                Err(err) => Err(err.into()),
            }
        }
    }

    pub trait ExpectLazy<T> {
        fn expect_lazy<F>(self, fun: F) -> T
        where
            F: FnOnce() -> String;
    }

    impl<T, E> ExpectLazy<T> for Result<T, E> {
        fn expect_lazy<F>(self, fun: F) -> T
        where
            F: FnOnce() -> String,
        {
            match self {
                Ok(ok) => ok,
                Err(_) => panic!("{}", fun()),
            }
        }
    }

    impl<T> ExpectLazy<T> for Option<T> {
        fn expect_lazy<F>(self, fun: F) -> T
        where
            F: FnOnce() -> String,
        {
            match self {
                Some(ok) => ok,
                None => panic!("{}", fun()),
            }
        }
    }

    #[cfg(test)]
    mod test_error_into {
        use super::ErrorInto;
        struct Error(String);

        impl Into<Error> for String {
            fn into(self) -> Error {
                Error(self)
            }
        }

        fn into_test() -> Result<String, String> {
            Err("err".to_string())
        }

        fn test_inot_call() -> Result<String, Error> {
            into_test().err_into()
        }

        #[test]
        fn testing() {
            let res = test_inot_call();
            assert!(res.is_err())
        }
    }
}

pub mod str {
    pub trait SplitToVec {
        fn split_to_vec(&self, pattern: impl Into<String>) -> Vec<String>;
    }

    impl SplitToVec for String {
        fn split_to_vec(&self, pattern: impl Into<String>) -> Vec<String> {
            self.split(&pattern.into())
                .map(|str| str.to_string())
                .collect()
        }
    }

    impl SplitToVec for &str {
        fn split_to_vec(&self, pattern: impl Into<String>) -> Vec<String> {
            self.split(&pattern.into())
                .map(|str| str.to_string())
                .collect()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        pub fn should_split() {
            let res = "test:ok".split_to_vec(":");
            assert_eq!("test", res[0]);
            assert_eq!("ok", res[1]);
        }
    }

}

pub mod prelude {
    pub use super::{
        lock::LockUnsafe,
        block::Block,
        error::ErrorToString,
        error::ErrorInto,
        error::ExpectLazy,
        str::SplitToVec
    };
}