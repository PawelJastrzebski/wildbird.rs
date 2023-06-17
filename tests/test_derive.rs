#[cfg(test)]
#[allow(dead_code)]
/// Only for debugging
/// use:  cargo expand --test test_derive
mod debug {
    use std::sync::Arc;

    #[derive(Debug)]
    struct Foo {
        test: Arc<Option<String>>,
        component_name: String,
    }

    impl Foo {
        pub fn test_expand_derive(&self) {
            println!("Hello");
        }
    }

}