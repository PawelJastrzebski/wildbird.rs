#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use wildbird::derive::*;

    #[derive(Service)]
    struct HelloService {
        // #[inject]
        test: Arc<Option<String>>,
        // #[inject("env:path", default = "test")]
        component_name: String,
    }

    #[ServiceConstruct]
    fn hello_init() -> HelloService {
        HelloService {
            test: Arc::new(Some("".to_string())),
            component_name: "Test".to_string(),
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
