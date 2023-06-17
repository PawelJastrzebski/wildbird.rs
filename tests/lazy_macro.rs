#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

/// use:  cargo expand --test test_lazy

mod lazy {
    use std::io::{Read, Write};
    use std::ops::Deref;
    use std::sync::Arc;
    use std::time::Duration;

    use wildbird::derive::*;
    use wildbird::Lazy;

    #[lazy]
    fn app_path() -> String {
        std::env::var("PWD").expect("env:PWD not found")
    }

    #[lazy(name = "PATH")]
    fn app_path_2() -> String {
        std::env::var("PWD").expect("env:PWD not found")
    }


    #[test]
    pub fn should_derive_lazy() {
        let path = APP_PATH.clone();
        println!("app_path: {path}");
        println!("APP_PATH: {APP_PATH}");
        println!("PATH: {PATH}");
    }
}