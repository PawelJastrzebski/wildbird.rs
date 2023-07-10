#![doc = include_str!("../README.md")]

mod lazy;
mod callback;
mod block;
mod lock;

/// Prelude Module
pub mod prelude;

/// Macro System
pub extern crate wildbird_macro_derive as derive;

pub use self::lazy::Lazy;
pub use self::callback::Callback;
pub use self::block::Block;

/// Private Module
/// > **Don't use in your code directly**
pub mod private;

/// Service Trait
pub trait Service {
    type Service;
    fn construct() -> Self::Service;
}