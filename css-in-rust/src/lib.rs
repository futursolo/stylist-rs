// Copyright © 2020 Lukas Wagner

#[macro_use]
extern crate lazy_static;
#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(all(test, target_arch = "wasm32"))]
extern crate wasm_bindgen_test;

pub mod bindings;
mod parser;
pub mod style;
