#![allow(dead_code, unused_variables, non_snake_case)]
use std::time::Duration;
use wildbird::*;

mod callback {
    use super::*;

    async fn db_call() -> String {
        std::thread::sleep(Duration::from_millis(10));
        String::from(format!("{}", "data"))
    }

    async fn init(callback: Callback<String>) -> () {
        let data = db_call().await;
        callback.call(data);
    }

    fn init_callback() -> String {
         wildbird::private::block_callback(init)
    }

    static INIT_LAZY: wildbird::Lazy<String> = wildbird::private::lazy_construct(init_callback);

    #[test]
    pub fn should_derive_lazy() {
        print!("Result: {INIT_LAZY}");
    }
}
