[![Build Status](https://travis-ci.com/lukidoescode/css-in-rust.svg?branch=master)](https://travis-ci.com/lukidoescode/css-in-rust)

# CSSinRust

CSSinRust delivers a new way of implementing CSS styling in web-sys applications.
It's aim is to make writing web frontends in Rust attractive by providing a way to style on a component level. The library is implemented so that it could in theory work with any framework. However, right now there is only an implementation for yew. Pull requests are very welcome, whether for improving code quality, integration solutions or functionality.

Please be aware that this project is still under heavy development and that future changes might break your code. I'm still not sure about the overall design but I needed something like this and I'm sure some other people do as well.

If you'd like to be kept up to date or you'd like to support my work please visit me on those platforms:

- [Twitter](https://twitter.com/lukidoescode)
- [Patreon](https://www.patreon.com/lukaswagner)

# Syntax

Currently there is only support for a very basic set of syntax. Even though the syntax is very similar to CSS there are a few particularities in CSSinRust which are inspired by SASS and styled-components in JS.

Here is how a basic style would get defined.

```rust
let style = match css_in_rust::Style::create(
    "Component", // The class prefix
    // The actual css
    r#"
    background-color: red;

    .nested {
        background-color: blue;
        width: 100px
    }"#,
) {
    Ok(style) => style,
    Err(error) => {
        panic!("An error occured while creating the style: {}", error);
    }
};
```

So everything that is not in a conditioned block will be applied to the Component the class of this style is applied to. How that happens depends on the framework to use. Below there are examples for the supported frameworks.

The Style you put in will get parsed and converted to actual CSS and automatically appended to the head of your HTML document.

You may also use the `&` identifier in order to use CSS selectors or pseudo classes on the styled element:

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

Please be aware that right now, CSSinRust will not parse the name of the animation in order to make it unique. If you need that feature please upvote the issue or open a new one if there is none already.

There is also media query support now!

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

# Integrations

## Seed

In order to enable all yew integration use the feature `seed_integration` for CSSinRust in your `Cargo.toml`. Then create a style and use it with yew like this:

```rust
pub(crate) struct Model {
    pub style: css_in_rust::Style,
}

impl Default for Model {
    fn default() -> Self {
        let style = match css_in_rust::Style::create(
            String::from("Component"),
            String::from(
                r#"
                background-color: #303040;
                color: #DDDDDD;
                padding: 5px;
                &:hover {
                    background-color: #606072;
                }
                "#,
            ),
        ) {
            Ok(style) => style,
            Err(error) => {
                panic!("An error occured while creating the style: {}", error);
            }
        };
        Self {
            style: style,
        }
    }
}

#[derive(Clone)]
pub(crate) enum Msg {
}

pub(crate) fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
}

pub(crate) fn view(model: &Model) -> impl View<Msg> {
    div![
        model.style.clone(),
        "Hello, World"
    ]
}
```

## Yew

In order to enable all yew integration use the feature `yew_integration` for CSSinRust in your `Cargo.toml`. Then create a style and use it with yew like this:

```rust
impl Component for HelloComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = match css_in_rust::Style::create(
            "Component",
            "background-color: #505050;",
        ) {
            Ok(style) => style,
            Err(error) => {
                panic!("An error occured while creating the style: {}", error);
            }
        };
        HelloComponent {
            style,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {<div class=self.style.clone()>{"Hello World!"}</div>}
    }
}
```

### CSSinRust Versions and Corresponding Yew Versions

| Yew Version | CSSinRust Version |
| ----------- | ----------------- |
| 0.14.x      | 0.2.2             |
| 0.15.x      | 0.3.x             |
| 0.16.x      | 0.4.x             |
| 0.17.x      | 0.5.x             |
