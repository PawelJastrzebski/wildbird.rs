
#[cfg(all(feature = "timed-log", not(feature = "timed-tracing")))]
pub use log::info as _print_timed;

#[cfg(feature = "timed-tracing")]
pub use tracing::info as _print_timed;

#[macro_export]
macro_rules! timed {
        ($name:literal $($code:tt)*) => {
            let _now = std::time::Instant::now();
            $($code)*
            _print_timed!("{}: took {}ms", format!($name) , _now.elapsed().as_millis());
        };

        ($($code:tt)*) => {
            let _now = std::time::Instant::now();
            $($code)*
            _print_timed!("timed: took {}ms" , _now.elapsed().as_millis());
        };
    }
pub use timed;

#[macro_export]
macro_rules! timed_return {
        ($name:literal $($code:tt)*) => {
            {
                let _now = std::time::Instant::now();
                let r = {$($code)*};
                _print_timed!("{}: took {}ms", format!($name) , _now.elapsed().as_millis());
                r
            }
        };

        ($($code:tt)*) => {
            {
                let _now = std::time::Instant::now();
                let r = {$($code)*};
                _print_timed!("timed: took {}ms" , _now.elapsed().as_millis());
                r
            }
        };
    }
pub use timed_return;

#[cfg(all(feature = "timed-log", test))]
mod timed_test_log {
    use crate::metric::*;

    fn init_log() {
        use simplelog::*;
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }

    fn wait(millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    #[test]
    pub fn timed_log() {
        init_log();
        timed! {
            wait(10);
        }
        timed! { "label"
            wait(11);
        }

        let _one = timed_return! {
            wait(1);
            1
        };
        let _two = timed_return! { "label"
            wait(2);
            2
        };
        timed_return! { "label"
            wait(3);
            3
        };
    }
}

#[cfg(all(feature = "timed-tracing", test))]
mod timed_test_tracing {
    use crate::metric::*;

    fn init_tracing() {
        tracing_subscriber::fmt()
            .compact()
            .with_target(false)
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    fn wait(millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    #[test]
    fn timed_tracing() {
        init_tracing();
        timed! {
            wait(10);
        }
        timed! { "label"
            wait(11);
        }

        let _one = timed_return! {
            wait(1);
            1
        };
        let _two = timed_return! { "label"
            wait(2);
            2
        };
        timed_return! { "label"
            wait(3);
            3
        };
    }
}
