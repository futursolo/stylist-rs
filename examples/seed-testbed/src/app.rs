// Copyright © 2020 Lukas Wagner

extern crate css_in_rust;

use css_in_rust::Style;
use seed::{prelude::*, *};

pub(crate) struct Model {
    pub val: i32,
    pub style: Style,
}

impl Default for Model {
    fn default() -> Self {
        let style = match Style::create(
            String::from("App"),
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
            val: 0,
            style: style,
        }
    }
}

#[derive(Clone)]
pub(crate) enum Msg {
    Increment,
}

pub(crate) fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
    }
}

pub(crate) fn view(model: &Model) -> impl View<Msg> {
    button![
        model.style.clone(),
        simple_ev(Ev::Click, Msg::Increment),
        format!("Hello, World × {}", model.val)
    ]
}
