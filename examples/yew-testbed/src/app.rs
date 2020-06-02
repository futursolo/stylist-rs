// Copyright © 2020 Lukas Wagner

extern crate css_in_rust;

use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    style: Style,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = match Style::create("App", include_str!("app.scss")) {
            Ok(style) => style,
            Err(error) => {
                panic!("An error occured while creating the style: {}", error);
            }
        };
        App { style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {<div class=self.style.clone()>
            {"The quick brown fox jumps over the lazy dog"}
            <div class="on-da-inside">{"The quick brown fox jumps over the lazy dog"}</div>
        </div>}
    }
}
