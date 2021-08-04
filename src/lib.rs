#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]

mod ast;
#[doc(hidden)]
pub mod bindings;
mod error;
mod parser;
mod registry;
pub mod style;
mod utils;

pub use error::{Error, Result};
pub use style::Style;
