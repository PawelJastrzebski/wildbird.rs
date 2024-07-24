use std::sync::Arc;
/// Only for debugging
/// use:  cargo expand --test inject_test
use wildbird::{prelude::*, Inject};

#[service(construct = "init")]
struct A {
    b: Arc<B>,
}
impl A {
    fn init() -> Self {
        Self { b: Inject() }
    }

    pub fn hello_a(&self, text: &str) {
        println!("hello A {text}");
        self.b.hello_b(text);
    }
}

#[service(construct = "init")]
struct B {
    a: Lazy<A>,
}
impl B {
    fn init() -> Self {
        Self { a: Inject() }
    }

    pub fn hello_b(&self, text: &str) {
        println!("hello B {text}");
        self.a.hello_a("Hello A from B")
    }
}

#[service(construct = "init")]
struct C {
    d: Arc<D>,
}
impl C {
    fn init() -> Self {
        Self { d: Inject() }
    }

    pub fn hello_c(&self, text: &str) {
        println!("hello C {text}");
        self.d.hello_d(text);
    }
}

#[service(construct = "init")]
struct D {}
impl D {
    fn init() -> Self {
        Self {}
    }

    pub fn hello_d(&self, text: &str) {
        println!("hello D {text}");
    }
}

#[test]
pub fn should_inject() {
    {
        let c: &C = Inject();
        c.hello_c("ok1");
    }
    {
        let c: Arc<C> = Inject();
        c.hello_c("ok1");
    }
    {
        let c: Lazy<C> = Inject();
        c.hello_c("ok1");
    }
}

#[test]
#[should_panic]
pub fn should_detect_cilcular_dependency() {
    let service_a: &A = Inject();
    service_a.hello_a("ok1");
}
