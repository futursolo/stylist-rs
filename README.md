# Stylist

[![Run Tests & Publishing](https://github.com/futursolo/stylist-rs/actions/workflows/everything.yml/badge.svg)](https://github.com/futursolo/stylist-rs/actions/workflows/everything.yml)
[![crates.io](https://img.shields.io/crates/v/stylist)](https://crates.io/crates/stylist)
[![docs.rs](https://docs.rs/stylist/badge.svg)](https://docs.rs/stylist/)

Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.

This is a fork of [css-in-rust](https://github.com/lukidoescode/css-in-rust).

## Install

Add the following to your `Cargo.toml`:

```toml
stylist = "0.9"
```

## Usage

For detailed usage, please see
[documentation](https://docs.rs/stylist/).

### Yew Integration

To style your component, you can use `styled_component` attribute with `css!`
macro.

```rust
use stylist::yew::styled_component;

#[styled_component(MyStyledComponent)]
fn my_styled_component() -> Html {
    html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
}
```

### Standalone

To create a stylesheet, you can use `style!`:

```rust
use stylist::style;

let style = style!(
   // A CSS string literal
   r#"
       background-color: red;

       .nested {
           background-color: blue;
           width: 100px
       }
   "#
).expect("Failed to mount style");

// stylist-uSu9NZZu
println!("{}", style.get_class_name());
```

### Runtime Style

If you want to parse a string into a style at runtime, you can use `Style::new`:

```rust
use stylist::Style;

let style_str = r#"
    background-color: red;

    .nested {
        background-color: blue;
        width: 100px
    }
"#;

let style = Style::new(style_str).expect("Failed to create style");

// stylist-uSu9NZZu
println!("{}", style.get_class_name());
```

### Theming

There's theming example using
[Yew Context API](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-context).
