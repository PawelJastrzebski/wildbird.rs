#![allow(dead_code, unused_variables, non_snake_case)]

mod callback {
    use std::time::Duration;

    use wildbird::Lazy;
    use wildbird::private::*;

    async fn init(callback: impl Fn(String)) -> () {
        std::thread::sleep(Duration::from_millis(10));
        callback("dONE".to_string())
    }

    fn init_callback() -> String {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        std::thread::spawn(move || {
            block(init(|v: String| {
                let _ = tx.send(v);
            }));
        });
        rx.recv().unwrap()
    }

    static INIT_LAZY: Lazy<String> = wildbird::private::lazy_construct(init_callback);

    #[test]
    pub fn should_derive_lazy() {
        print!("Result: {INIT_LAZY}");
    }
}
