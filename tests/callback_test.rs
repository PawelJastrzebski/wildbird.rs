#![allow(dead_code, non_snake_case)]

use std::time::Duration;
use wildbird::*;
use wildbird::derive::var;

/// use:  cargo expand --test callback_test
mod callback_tokio {
    use super::*;

    async fn db_call() -> String {
        std::thread::sleep(Duration::from_millis(10));
        String::from(format!("{}", "data"))
    }

    #[var]
    async fn callback(call: Callback<Option<String>>) {
        let data = db_call().await;
        call.call(Some(data));
    }

    #[var]
    async fn number(cal: Callback<i32>) {
        cal.call(12);
        cal.call(13);
    }

    #[test]
    #[cfg(not(feature = "tokio"))]
    pub fn should_derive_lazy() {
        println!("Option: {:?}", *CALLBACK);
        println!("Number: {:?}", *NUMBER);
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    pub async fn should_derive_lazy() {
        println!("Option: {:?}", *CALLBACK);
        println!("Option: {:?}", *CALLBACK.instance());
        println!("Number: {:?}", *NUMBER);
    }
}