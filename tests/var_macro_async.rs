#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

/// use:  cargo expand --test var_macro_async
mod lazy {
    use std::io::{Read, Write};
    use std::ops::Deref;
    use std::sync::Arc;
    use std::time::Duration;

    use wildbird::derive::*;
    use wildbird::Lazy;

    #[var(name = "DB")]
    async fn connect_db() -> String {
        println!("start");
        std::thread::sleep(Duration::from_millis(100));
        println!("sleep");
        std::thread::sleep(Duration::from_millis(100));
        println!("sleep");
        std::thread::sleep(Duration::from_millis(100));
        println!("ready");
        String::from("Result from db")
    }

    #[test]
    pub fn should_derive_lazy() {
        println!("DB: {}", DB)
    }

    #[tokio::test]
    async fn should_derive_lazy_tokio() {
        println!("DB: {}", DB);
        println!("DB: {}", DB);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn should_derive_lazy_tokio_pool() {
        println!("DB: {}", DB);
        println!("DB: {}", DB);
    }
}