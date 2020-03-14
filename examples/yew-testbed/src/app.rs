// Copyright © 2020 Lukas Wagner

extern crate css_in_rust;

use css_in_rust::style::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

const KEY: &str = "css-in-rust/yew-testbed";

pub struct App {
    style: Style,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("App"),
            String::from(
                r#"
                background-color: red;
                .on-da-inside {
                    background-color: blue;
                    width: 100px
                }
                "#,
            ),
        );
        App { style }
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
