#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![cfg_attr(doc, feature(doc_cfg))]

pub mod ast;
pub mod bindings;
#[doc(hidden)]
pub mod style;

#[cfg(target_arch = "wasm32")]
#[path = "arch_wasm.rs"]
mod arch;
mod registry;
mod utils;

pub use style::Style;
