// Copyright © 2020 Lukas Wagner

//! Yew integration module.
//! The user doesn't need to do anything but to put a style into the class of a
//! yew component.

#[cfg(target_arch = "wasm32")]
extern crate yew;

#[cfg(target_arch = "wasm32")]
use super::super::style::Style;
#[cfg(target_arch = "wasm32")]
use yew::virtual_dom::Classes;

#[cfg(target_arch = "wasm32")]
impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.class_name.as_str());
        classes
    }
}
