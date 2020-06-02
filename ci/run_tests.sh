#!/bin/bash

# This is blocked by https://github.com/rustwasm/wasm-pack/issues/698
#wasm-pack test --node -- --features std_web
cargo test
