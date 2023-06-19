#![allow(dead_code, unused_variables, non_snake_case)]

/// use:  cargo expand --test test_lazy

mod lazy {
    use wildbird::derive::*;

    #[var]
    fn app_path() -> String {
        std::env::var("PWD").expect("env:PWD not found")
    }

    #[var(name = "PATH")]
    fn app_path_2() -> Option<u32> {
        Some(32)
    }

    #[test]
    pub fn should_derive_lazy() {
        let path = APP_PATH.clone();
        println!("app_path: {path}");
        println!("APP_PATH: {APP_PATH}");
        println!("Number: {:?}", PATH);
        println!("Number: {}", PATH.unwrap());
    }
}