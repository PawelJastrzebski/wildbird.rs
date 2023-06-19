#![allow(dead_code, unused_variables, non_snake_case)]

mod service {
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
        use std::ops::Deref;
        let x = HelloService.deref();
        x.hello("0");
        HelloService.hello("1");
    }
}