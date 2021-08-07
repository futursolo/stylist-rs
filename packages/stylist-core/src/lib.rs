#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]

pub mod ast;
#[doc(hidden)]
pub mod bindings;
#[doc(hidden)]
pub mod style;

mod registry;
mod utils;

pub use style::Style;
