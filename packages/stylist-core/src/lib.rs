#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]

pub mod ast;
#[doc(hidden)]
pub mod bindings;
#[doc(hidden)]
pub mod style;

#[cfg(target_arch = "wasm32")]
#[path = "arch_wasm.rs"]
mod arch;
mod registry;
mod utils;

pub use style::Style;
