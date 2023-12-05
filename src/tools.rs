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
        #[inline(always)]
        fn block(self) -> F::Output {
            crate::private::block(self.into_future())
        }
    }

    /// Runs code in async environment
    ///
    /// # Example
    /// ```rust
    /// use wildbird::prelude::*;
    ///
    /// async fn fetch_from_api() -> String {
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    ///     String::from("Ok")
    /// }
    ///
    /// trait Test {
    ///     fn get_response() -> String {
    ///         async_block! {
    ///             let res = fetch_from_api().await;
    ///             assert_eq!("Ok", res);
    ///             res
    ///         }
    ///     }
    /// }
    ///
    /// # struct TestImpl;
    /// # impl Test for TestImpl {}
    /// # TestImpl::get_response();
    /// ```
    #[macro_export]
    macro_rules! async_block {
        ($($e:tt)*) => {
            async {
                $($e)*
            }.block()
        }
    }
    pub use async_block;

    #[macro_export]
    macro_rules! async_block_unwind {
        ($($e:tt)*) => {{
            std::panic::catch_unwind(|| {
                async {
                    $($e)*
                }.block()
            }).ok();
        }}
    }
    pub use async_block_unwind;

    #[macro_export]
    macro_rules! async_closure_unwind {
        ($($e:tt)*) => {(
            move || {
                std::panic::catch_unwind(|| {
                    async {
                        $($e)*
                    }.block()
                }).ok();
            }
        )}
    }
    pub use async_closure_unwind;

    #[cfg(test)]
    mod test_block {
        use crate::prelude::*;
        use std::time::{Duration, Instant};

        async fn fetch_from_api() -> String {
            std::thread::sleep(Duration::from_millis(100));
            String::from("Api respone")
        }

        #[test]
        fn should_block() {
            assert_eq!("Api respone", fetch_from_api().block());
        }

        #[test]
        fn async_closuer_unwind() {
            std::thread::spawn(async_closure_unwind! {
                    assert_eq!("Api respone", fetch_from_api().await);
                    println!("ok");
                    panic!("error");
            })
            .join()
            .unwrap();
            println!("async closure prevented panic");
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn should_block_async() {
            println!("Res: {}", fetch_from_api().block());
        }

        #[test]
        fn should_block_async_block() {
            let action = async {
                let data1 = fetch_from_api().await;
                let data2 = fetch_from_api().await;
                format!("{data1}, {data2}")
            };

            let macro_result = async_block! {
                let data1 = fetch_from_api().await;
                let data2 = fetch_from_api().await;
                format!("{data1}, {data2}")
            };

            assert_eq!("Api respone, Api respone", action.block());
            assert_eq!("Api respone, Api respone", macro_result);
        }

        #[ignore = "only for manual testing"]
        #[tokio::test(flavor = "multi_thread")]
        async fn performance_test() {
            async fn async_fn(i: i32) -> i32 {
                10 + i
            }

            //warm up
            async_fn(0).await;
            async_fn(0).block();

            // standard async
            let now = Instant::now();
            for i in 0..1_000_000 {
                async_fn(i).await;
            }
            let async_took = now.elapsed().as_millis();

            // async block (recomended)
            let now = Instant::now();
            async_block! {
                for i in 0..1_000_000 {
                    async_fn(i).await;
                }
            }
            let async_block = now.elapsed().as_millis();

            // block in loop (not recomended)
            let now = Instant::now();
            for i in 0..1_000_000 {
                async_fn(i).block();
            }
            let async_block_in_loop = now.elapsed().as_millis();

            println!("await took: {} ms", async_took);
            println!("async_block took: {} ms", async_block);
            println!("async_block in loop took: {} ms", async_block_in_loop);
        }
    }
}

pub mod unwind {

    #[macro_export]
    macro_rules! unwind {
        ($($e:tt)*) => {
            std::panic::catch_unwind($($e)*)
        }
    }
    pub use unwind;

    #[cfg(test)]
    mod test_unwind {
        use super::*;

        #[test]
        fn test_unwind_closure_success() {
            let x = String::from("test");
            if let Some(y) = unwind!(|| x.to_owned() + "ok").ok() {
                println!("result: y={y} x=moved")
            } else {
                println!("operation failed")
            }
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
    mod test_lock {
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

    pub trait InspectError<T, E> {
        fn inspect_error<F>(self, fun: F) -> Result<T, E>
        where
            F: FnOnce(&E);
    }

    impl<T, E> InspectError<T, E> for Result<T, E> {
        fn inspect_error<F>(self, fun: F) -> Result<T, E>
        where
            F: FnOnce(&E),
        {
            if let Err(err) = self.as_ref() {
                fun(err)
            }

            self
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

        fn test_into_call() -> Result<String, Error> {
            into_test().err_into()
        }

        #[test]
        fn testing() {
            let res = test_into_call();
            assert!(res.is_err())
        }
    }

    #[cfg(test)]
    mod test_error_inspect {
        use super::InspectError;

        fn into_test() -> Result<String, String> {
            Err("ERROR".to_string())
        }

        #[test]
        fn testing() {
            let res = into_test();
            assert!(res.is_err());
            let _ = res.inspect_error(|e| println!("error inpsect ok: {e}"));
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
    mod test_str {
        use super::*;

        #[test]
        pub fn should_split() {
            let res = "test:ok".split_to_vec(":");
            assert_eq!("test", res[0]);
            assert_eq!("ok", res[1]);
        }
    }
}

mod math {
    pub trait Round<T> {
        fn round_precision(self, digits: usize) -> T;
    }

    impl Round<f64> for f64 {
        #[inline(always)]
        fn round_precision(self, digits: usize) -> f64 {
            let multi = 10.0_f64.powf(digits as f64);
            (self * multi).round() / multi
        }
    }

    impl Round<f32> for f32 {
        #[inline(always)]
        fn round_precision(self, digits: usize) -> f32 {
            let multi = 10.0_f32.powf(digits as f32);
            (self * multi).round() / multi
        }
    }

    #[cfg(test)]
    mod test_math {
        use super::*;

        #[test]
        pub fn should_round() {
            assert_eq!(0.0, 0.00300.round_precision(1));
            assert_eq!(0.1, 0.10300.round_precision(1));
            assert_eq!(0.0, 0.33333.round_precision(0));
            assert_eq!(0.3, 0.33333.round_precision(1));
            assert_eq!(0.33, 0.33333.round_precision(2));
            assert_eq!(0.333, 0.33333.round_precision(3));
            assert_eq!(0.334, 0.33355.round_precision(3));
            assert_eq!(
                0.02,
                0.01499999999999999944488848768742172978818416595458984375_f64.round_precision(2)
            );
            assert_eq!(0.01, 0.014999999_f64.round_precision(2));
        }
    }
}

mod into {
    pub trait IntoOption<T> {
        fn none(self) -> Option<T>;
        fn some(self) -> Option<T>;
    }
    
    impl<T> IntoOption<T> for T {
        fn none(self) -> Option<T> {
            None
        }
    
        fn some(self) -> Option<T> {
            Some(self)
        }
    }
    
    pub trait IntoResult<T> {
        fn ok<E>(self) -> Result<T, E>;
        fn err<S, IE>(self) -> Result<S, IE> where T: Into<IE>;
    }
    
    impl<T> IntoResult<T> for T {

        fn err<S, IE>(self) -> Result<S, IE> where T: Into<IE> {
            Err(self.into())
        }
    
        fn ok<E>(self) -> Result<T, E> {
            Ok(self)
        }
      
    }
}

pub mod prelude {
    pub use super::{
        block::async_block, block::async_block_unwind, block::async_closure_unwind, block::Block,
        error::ErrorInto, error::ErrorToString, error::ExpectLazy, error::InspectError,
        lock::LockUnsafe, math::Round, str::SplitToVec, into::IntoOption, into::IntoResult
    };
}
