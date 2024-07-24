use std::{sync::Arc, time::Duration};

use wildbird::prelude::*;
/// Only for debugging
/// use:  cargo expand --test inject_construct_test
#[service(constructor = "init")]
struct InnerService {
    component_name: String,
}

impl InnerService {
    fn init() -> Self {
        Self {
            component_name: "Inner".to_string(),
        }
    }
}

#[allow(dead_code)]
#[service]
struct HelloService {
    component_name: String,
    inner: Arc<InnerService>,
}

#[service(construct)]
fn hello_init(inner: Arc<InnerService>) -> HelloService {
    HelloService {
        inner,
        component_name: "Hello".to_string(),
    }
}

impl HelloService {
    pub fn hello(&self) -> String {
        println!("Hello with: {}", self.inner.component_name);
        self.inner.component_name.clone()
    }
}


#[test]
pub fn should_inject_through_construct() {
    let inner = HelloService.hello();
    assert_eq!("Inner", inner)
}

#[allow(dead_code)]
#[service]
struct AsyncService {
    pub hello_service: Arc<HelloService>,
}

#[service(construct)]
async fn async_init(hello_service: Arc<HelloService>) -> AsyncService {
    std::thread::sleep(Duration::from_millis(400));
    AsyncService {
        hello_service
    }
}

#[test]
pub fn should_inject_through_construct_async() {
    let inner = AsyncService.hello_service.hello();
    assert_eq!("Inner", inner)
}