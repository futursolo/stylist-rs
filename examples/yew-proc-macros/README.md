# Procedural Macro Example

1. Install [Trunk](https://trunkrs.dev) and [Rust](https://rustup.rs)
2. `trunk serve --open`

# Compile for the smallest size

Add the following lines to `Cargo.toml` of the workspace:

```toml
[profile.release]
opt-level = 'z'
lto = true
```

Open with `trunk serve --open --release`
