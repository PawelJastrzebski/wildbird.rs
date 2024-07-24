#![allow(dead_code, unused_variables, non_snake_case)]
/// Only for debugging
/// use:  cargo expand --test s_construct_method
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
        let drf: &HelloService = HelloService.to_ref();
        let drf: wildbird::Lazy<HelloService> = HelloService.clone_lazy();
        let drf: std::sync::Arc<HelloService> = HelloService.instance();
        HelloService.hello("1");
        drf.hello("0");
    }
}