#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::sync::Arc;
    use wildbird::derive::*;
    use wildbird::Service;

    #[service(construct = "hello_init")]
    struct HelloService {}

    impl HelloService {
        pub fn hello(&self, text: &str) {
            println!("hello {text}")
        }

        fn hello_init() -> HelloService {
            println!("init once");
            HelloService {}
        }
    }

    #[test]
    fn should_derive_Service() {
        HelloService.hello("1");
    }
}
