#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(non_snake_case)]

mod error;
pub use error::{Error, Result};
pub mod ast;
mod parser;
