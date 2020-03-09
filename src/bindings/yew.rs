// Copyright © 2020 Lukas Wagner

extern crate yew;

use super::super::style::Style;
use yew::virtual_dom::Classes;

impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.class_name.as_str());
        classes
    }
}
