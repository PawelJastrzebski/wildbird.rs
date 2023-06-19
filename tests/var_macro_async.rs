#![allow(dead_code, unused_variables, non_snake_case)]

/// use:  cargo expand --test var_macro_async
mod lazy {
    use std::{time::Duration, ops::Deref};
    use wildbird::derive::*;

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
    pub fn should_display_and_debug() {
        let _ = DB.clone();
        println!("DB: {}", DB);
        println!("DB: {:?}", DB);
    }

    #[test]
    pub fn test_deref_and_clone() {
        println!("DB: {}", DB);
        let deref: &String = DB.deref();
        let clone: wildbird::Lazy<String>= DB.clone();
        assert_eq!(deref, clone.deref());
    }

    #[tokio::test]
    async fn should_derive_lazy_tokio() {
        println!("DB: {}", DB);
        println!("DB: {}", DB);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn should_derive_lazy_tokio_pool() {
        println!("DB: {}", DB);
        println!("DB: {DB}");
    }
}