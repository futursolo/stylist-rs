#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

mod error;
pub use error::{Error, Result};
pub mod ast;
mod parser;

#[cfg_attr(documenting, doc(cfg(feature = "yew_integration")))]
#[cfg(feature = "yew_integration")]
mod yew;
