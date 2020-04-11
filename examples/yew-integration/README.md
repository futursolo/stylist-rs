# Quickstart

`rustup update`

`rustup target add wasm32-unknown-unknown`

`cargo install cargo-make`

Run `cargo make build` in a terminal to build the app, and `cargo make serve` to start a dev server
on `127.0.0.1:8000`.

If you'd like the compiler automatically check for changes, recompiling as
needed, run `cargo make watch` instead of `cargo make build`.