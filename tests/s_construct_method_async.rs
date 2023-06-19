#![allow(dead_code, unused_variables, non_snake_case)]

mod service {
    use std::thread::sleep;
    use std::time::Duration;
    use wildbird::derive::*;

    #[service(construct = "async hello_init")]
    struct HelloService {}

    impl HelloService {
        async fn hello_init() -> HelloService {
            sleep(Duration::from_millis(200));
            println!("init once");
            HelloService {}
        }

        pub fn hello(&self) {
            println!("hello !!")
        }
    }

    #[test]
    fn should_derive_Service() {
        HelloService.hello();
    }
}