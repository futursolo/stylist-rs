# Stylist

Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.

This is a fork of [css-in-rust](https://github.com/lukidoescode/css-in-rust).

## Usage

To create a stylesheet, use `Style::create`:

```rust
use stylist::Style;

let style = Style::create(
    // The class prefix
    "Component",

    // The actual css
    r#"
    background-color: red;

    .nested {
        background-color: blue;
        width: 100px
    }"#,
).expect("Failed to create style");
```

Everything that is not in a conditioned block will be applied to the Component
the class of this style is applied to. How that happens depends on the framework to use.
Below there are examples for the supported frameworks.

The Style you put in will get parsed and converted to actual CSS and automatically appended
to the head of your HTML document.

You may also use the `&` identifier in order to use CSS selectors or pseudo
classes on the styled element:

```css
&:hover {
  background-color: #d0d0d9;
}
```

You can also use other CSS rules, e.g. keyframes:

```css
@keyframes mymove {
  from {
    top: 0px;
  }
  to {
    top: 200px;
  }
}
```

```css
@media only screen and (max-width: 600px) {
  background-color: #303040;

  .nested {
    background-color: lightblue;
  }

  &:hover {
    background-color: #606072;
  }
}
```

## Yew Integration

To enable yew integration.

Then create a style and use it with yew like this:

```rust
use stylist::Style;

struct MyStyledComponent {
  style: Style,
}

impl Component for MyStyledComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = match Style::create(
            "styled-component",
            "background-color: #505050;",
        ).unwrap();
        Self {
            style,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<div class=self.style.clone()>{"Hello World!"}</div>}
    }
}
```
