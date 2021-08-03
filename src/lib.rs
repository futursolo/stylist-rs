#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]

mod ast;
#[doc(hidden)]
pub mod bindings;
mod parser;
pub mod style;
mod utils;

pub use style::Style;
