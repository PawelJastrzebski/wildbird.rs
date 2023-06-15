#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::sync::Arc;
    use wildbird::derive::*;

    #[service(construct = "init")]
    struct HelloService {}

    impl HelloService {
        pub fn hello(&self, text: &str) {
            println!("hello {text}")
        }

        fn init() -> HelloService {
            println!("init once");
            HelloService {}
        }
    }

    #[test]
    fn should_derive_Service() {
        HelloService.hello("1");

        let z = HelloService.instance();
        let z = HelloService.instance();
        z.hello("z 1");
        z.hello("z 2");
        z.hello("z 3");

        let work1 = std::thread::spawn(|| {
            HelloService.hello("work1")
        });

        let work2 = std::thread::spawn(|| {
            let i1 = HelloService.instance();
            i1.hello("work2 1");

            let i2 = i1.instance();
            i2.hello("work2 2")
        });

        work1.join().unwrap();
        work2.join().unwrap();

        z.hello("last");
    }
}
