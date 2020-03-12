// Copyright © 2020 Lukas Wagner

extern crate css_in_rust;

use css_in_rust::style::Style;
use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

const KEY: &str = "css-in-rust/yarn-testbed";

pub struct App {
    storage: StorageService,
    state: State,
    style: Style,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            value: "".into(),
            edit_value: "".into(),
        };
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
        App {
            storage,
            state,
            style,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {<div class=self.style.clone()>
            {"The quick brown fox jumps over the lazy dog"}
            <div class="on-da-inside">{"The quick brown fox jumps over the lazy dog"}</div>
        </div>}
    }
}
