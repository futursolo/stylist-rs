# Stylist

[![Run Tests & Publishing](https://github.com/futursolo/stylist-rs/actions/workflows/everything.yml/badge.svg)](https://github.com/futursolo/stylist-rs/actions/workflows/everything.yml)
[![crates.io](https://img.shields.io/crates/v/stylist)](https://crates.io/crates/stylist)
[![docs.rs](https://docs.rs/stylist/badge.svg)](https://docs.rs/stylist/)

Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.

This is a fork of [css-in-rust](https://github.com/lukidoescode/css-in-rust).

## Install

Add the following to your `Cargo.toml`:

```toml
stylist = "0.8"
```

## Usage

### Procedural Macros

To create a stylesheet, you can use [`style!`]:

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
);
```
### Style API

To create a stylesheet, use `Style::new`:

```rust
use stylist::Style;

let style = Style::new(
    // The actual css
    r#"
        background-color: red;

        .nested {
            background-color: blue;
            width: 100px
        }
    "#,
).expect("Failed to create style");
```

### YieldStyle API

Alternatively, any struct that implements `YieldStyle` trait can call
`self.style()` to get a `Style` instance.

```rust
use std::borrow::Cow;
use stylist::YieldStyle;

pub struct Component;

impl YieldStyle for Component {
    fn style_str(&self) -> Cow<'static, str> {
        "color: red;".into()
    }
}

impl Component {
    fn print_style(&self) -> Self {
        println!("{}", self.style().get_class_name());
    }
}
```

## Yew Integration

To enable yew integration. Enable feature `yew_integration` in `Cargo.toml`.

Then create a style and use it with yew like this:

```rust
use stylist::YieldStyle;

struct MyStyledComponent {}

impl Component for MyStyledComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<div class=self.style()>{"Hello World!"}</div>}
    }
}

impl YieldStyle for MyStyledComponent {
    fn style_str(&self) -> Cow<'static, str> {
        "color: red;".into()
    }
}
```

### Theming

There're theming examples using
[Yewdux](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-yewdux)
and [yewtil::store](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-agent).
