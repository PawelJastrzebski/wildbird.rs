#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use wildbird::derive::*;

    #[service]
    struct HelloService {
        test: Arc<Option<String>>,
        component_name: String,
    }

    #[service(construct)]
    fn hello_init() -> HelloService {
        HelloService {
            test: Arc::new(Some("".to_string())),
            component_name: "Hello".to_string(),
        }
    }

    impl HelloService {
        pub fn hello(&self) {
            println!("Hello");
        }
    }

    #[service]
    struct WorldService {
    }

    #[service(construct)]
    fn hello_init() -> WorldService {
        WorldService {}
    }

    impl WorldService {
        pub fn world(&self) {
            println!("World");
        }
    }

    #[test]
    fn should_derive_Service() {
        let t: Arc<HelloService> = HelloService.instance();
        let t: Arc<WorldService> = WorldService.instance();
        HelloService.hello();
        WorldService.world();
    }
}
