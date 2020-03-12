// Copyright © 2020 Lukas Wagner

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
#[cfg(test)]
extern crate wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
pub mod bindings;
#[cfg(target_arch = "wasm32")]
mod parser;
#[cfg(target_arch = "wasm32")]
pub mod style;
