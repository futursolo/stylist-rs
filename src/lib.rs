#![deny(clippy::all)]

mod ast;
#[doc(hidden)]
pub mod bindings;
mod parser;
pub mod style;
mod utils;

pub use style::Style;
