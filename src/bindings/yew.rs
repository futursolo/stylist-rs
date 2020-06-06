// Copyright © 2020 Lukas Wagner

//! Yew integration module.
//! The user doesn't need to do anything but to put a style into the class of a
//! yew component.

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
