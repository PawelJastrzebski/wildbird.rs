#![doc = include_str!("../README.md")]

mod callback;
mod inject;
mod lazy;
pub mod tools;

#[cfg(any(feature = "timed-log", feature = "timed-tracing", feature = "timed"))]
pub mod metric;
#[cfg(feature = "rayon")]
pub mod threads;
#[cfg(feature = "rayon")]
pub use rayon;

/// Prelude Module
pub mod prelude;

/// Macro System
pub extern crate wildbird_macro_derive as derive;

pub use self::callback::Callback;
pub use self::inject::Inject;
pub use self::lazy::Lazy;

/// Private Module
/// > **Don't use in your code directly**
pub mod private;

/// Service Trait
pub trait Service {
    type Service;
    fn construct() -> Self::Service;
}
