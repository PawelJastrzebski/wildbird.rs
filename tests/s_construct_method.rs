#![allow(dead_code, unused_variables, non_snake_case)]

mod service {
    use std::ops::Deref;
    use wildbird::derive::*;

    #[service(construct = "hello_init")]
    struct HelloService {}

    impl HelloService {
        fn hello_init() -> HelloService {
            println!("init once");
            HelloService {}
        }

        pub fn hello(&self, text: &str) {
            println!("hello {text}")
        }
    }

    #[test]
    fn should_derive_Service() {
        let drf: &HelloService = HelloService.deref();
        let drf: wildbird::Lazy<HelloService> = HelloService.clone();
        let drf: std::sync::Arc<HelloService> = HelloService.instance();
        HelloService.hello("1");
        drf.hello("0");
    }
}