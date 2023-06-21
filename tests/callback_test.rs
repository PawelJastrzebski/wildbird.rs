#![allow(dead_code, non_snake_case)]

use std::time::Duration;
use wildbird::*;
use wildbird::derive::*;

/// use:  cargo expand --test callback_test
mod callback {
    use super::*;

    async fn db_call() -> String {
        std::thread::sleep(Duration::from_millis(10));
        String::from(format!("{}", "data"))
    }

    #[var]
    async fn callback(callback1: Callback<Option<String>>) {
        let data = db_call().await;
        callback1.call(Some(data));
    }

    #[var]
    async fn number(cal: Callback<i32>) {
        cal.call(12);
        cal.call(13);
    }

    #[test]
    pub fn should_derive_lazy() {
        println!("Option: {:?}", CALLBACK);
        println!("Number: {:?}", NUMBER);
    }
}
