use std::borrow::Cow;

use stylist::{Style, StyleExt, YieldStyle};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use log::Level;

pub struct Inside {}

impl Component for Inside {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style()>
                {"The quick brown fox jumps over the lazy dog"}
            </div>
        }
    }
}

// You can implement YieldStyle for your component and call `.style()` in the render method.
impl YieldStyle for Inside {
    // Every `.style()` is called, this method will also be called.
    // So you can create style dynamically (theming).
    fn style_str(&self) -> Cow<'static, str> {
        r#"
            background-color: blue;
            width: 100px;
        "#
        .into()
    }
}

pub struct App {
    style: Style,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Alternatively, you can create Style manually during Component creation.
        let style = Style::new(
            r#"
                background-color: red;
            "#,
        )
        .unwrap();
        Self { style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                {"The quick brown fox jumps over the lazy dog"}
                <Inside />
            </div>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
