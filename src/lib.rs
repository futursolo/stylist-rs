#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]

mod ast;
#[doc(hidden)]
pub mod bindings;
mod error;
mod parser;
mod registry;
#[doc(hidden)]
pub mod style;
mod utils;
#[doc(hidden)]
pub mod yield_style;

pub use error::{Error, Result};
pub use style::Style;
pub use yield_style::YieldStyle;
