#![allow(dead_code, unused_variables, non_snake_case)]

/// use:  cargo expand --test s_macro_async
mod service {
    use std::sync::Arc;
    use std::time::Duration;
    use wildbird::derive::*;

    #[service]
    struct HelloService {
        name: String,
    }

    #[service(construct)]
    async fn hello_init() -> HelloService {
        std::thread::sleep(Duration::from_millis(100));
        HelloService {
            name: "Hello".to_string(),
        }
    }

    impl HelloService {
        pub fn hello(&self) {
            println!("Hello");
        }
    }

    #[test]
    fn should_derive_Service() {
        let t: Arc<HelloService> = HelloService.instance();
        HelloService.hello();
    }
}