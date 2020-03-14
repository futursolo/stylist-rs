extern crate css_in_rust;
extern crate log;
extern crate yew;

use css_in_rust::style::Style;
use log::trace;
use yew::App;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct CustomComponent {
    style: Style,
}

impl Component for CustomComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "CustomComponent",
            r#"
            background-color: red;
            .on-da-inside {
                background-color: blue;
                width: 100px
            }
            "#,
        );
        CustomComponent { style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {<div class=self.style.clone()>
            {"The quick brown fox jumps over the lazy dog"}
            <div class="on-da-inside">{"The quick brown fox jumps over the lazy dog"}</div>
        </div>}
    }
}
fn main() {
    web_logger::init();
    yew::start_app::<CustomComponent>();
}
