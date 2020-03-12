# CSSinRust

CSSinRust delivers a new way of implementing CSS styling in web-sys applications.
It's aim is to make writing web frontends in Rust attractive by providing a way to style on a component level. The library is implemented so that it could in theory work with any framework. However, right now there is only an implementation for yew. Pull requests are very welcome, whether for improving code quality, integration solutions or functionality.

Please be aware that this project is still under heavy development and that future changes might break your code. I'm still not sure about the overall design but I needed something like this and I'm sure some other people do as well.

If you'd like to be kept up to date or you'd like to support my work:

- [Twitter](https://twitter.com/lukidoescode)
- [Patreon](https://www.patreon.com/lukaswagner)

# Syntax

Currently there is only support for a very basic set of syntax. Even though the syntax is very similar to CSS there are a few particularities in CSSinRust which are inspired by SASS and styled-components in JS.

Here is how a basic style would get defined.

```rust
let style = css_in_rust::style::Style::create(
    String::from("Component"), // The class prefix
    String::from( // The actual css
        r#"
        background-color: red;

        .nested {
            background-color: blue;
            width: 100px
        }"#,
    )
)
```

So everything that is not in a conditioned block will be applied to the Component the class of this style is applied to. How that happens depends on the framework to use. Below there are examples for the supported frameworks.

The Style you put in will get parsed and converted to actual CSS and automatically appended to the head of your HTML document.

## Planned Syntactic Features

Things described here will happen earlier rather than later as soon as I find the time. However, pull requests are very welcome.

There are plans to add the `&` identifier so that things like these will be made possible:

```css
&:hover {
    background-color: #D0D0D9;
}
```

There are also plans to add in media query support like that:

```css
@media only screen and (max-width: 600px) {
    background-color: #303040;

    .nested {
        background-color: lightblue;
    }
}
```

Finally CSSinRust will be able to support CSS rules like that:

```css
@keyframes mymove {
    from {top: 0px;}
    to {top: 200px;}
}
```

# Integrations

## Yew

In order to use a created style in yew just insert it into the class attribute like that:

```rust
impl Component for HelloComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = css_in_rust::style::Style::create(
            String::from("Component"),
            String::from("background-color: #505050;"),
        );
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
