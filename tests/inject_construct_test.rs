use std::sync::Arc;

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
