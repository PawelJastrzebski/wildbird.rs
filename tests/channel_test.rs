#![allow(dead_code, unused_variables, non_snake_case)]

mod channel_impl {
    use std::sync::mpsc::*;
    use std::thread::spawn;

    pub struct Channel<T> {
        sender: SyncSender<T>,
    }

    impl<T: Send + Sync + 'static> Channel<T> {
        pub fn new(listener: fn(T) -> ()) -> Channel<T> {
            let (tx, rx) = sync_channel::<T>(20);

            spawn(move || {
                for data in rx.into_iter() {
                    listener(data);
                }
            });

            Channel { sender: tx }
        }

        pub fn send(&self, data: T) -> Result<(), SendError<T>> {
            self.sender.send(data)
        }
    }
}

mod channel_test {
    use std::time::Duration;

    use super::channel_impl::*;

    async fn listen(data: String) {
        println!("Data: {}", data)
    }

    fn listen_init(data: String) {
        wildbird::private::block(async { listen(data).await; })
    }

    fn init_chanel() -> Channel<String> {
        Channel::new(listen_init)
    }
    static TEST: wildbird::Lazy<Channel<String>> = wildbird::private::lazy_construct(init_chanel);

    #[test]
    pub fn should_derive_lazy() {
        TEST.send("Hello".to_string()).unwrap();
        std::thread::spawn(|| {
            TEST.send("Hello 1".to_string()).unwrap();
            TEST.send("Hello 2".to_string()).unwrap();
        });
        std::thread::sleep(Duration::from_millis(1))
    }
}
