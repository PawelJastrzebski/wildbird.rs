#![allow(dead_code, unused_variables, non_snake_case)]

/// use:  cargo expand --test var_macro_async
mod lazy {
    use std::time::Duration;
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
        println!("DB: {}", *DB);
        println!("DB: {:?}", DB);
    }

    #[test]
    pub fn test_deref_and_clone() {
        let get: &String = DB.to_ref();

        let clone: wildbird::Lazy<String>= DB.clone_lazy();
        assert_eq!(get, clone.to_ref());

        let deref: &String = &*DB;
        assert_eq!(get, deref);

        let reff: &String = DB.to_ref();
        assert_eq!(get, reff);
    }

    #[tokio::test]
    async fn should_derive_lazy_tokio() {
        println!("DB: {}", *DB);
        println!("DB: {}", DB);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn should_derive_lazy_tokio_pool() {
        println!("DB: {}", *DB);
        println!("DB: {DB}");
    }
}