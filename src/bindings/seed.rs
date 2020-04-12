// Copyright © 2020 Lukas Wagner

//! Seed integration module.
//! The user doesn't need to do anything but to add a style into a
//! seed component.

#[cfg(target_arch = "wasm32")]
extern crate seed;

#[cfg(target_arch = "wasm32")]
use super::super::style::Style;

#[cfg(target_arch = "wasm32")]
use seed::virtual_dom::{At, AtValue, Attrs, El, UpdateEl};

#[cfg(target_arch = "wasm32")]
impl<Ms> UpdateEl<El<Ms>> for Style {
    fn update(self, el: &mut El<Ms>) {
        let mut new_attrs = Attrs::empty();
        new_attrs.add(At::Class, self);
        el.attrs.merge(new_attrs);
    }
}

// #[cfg(target_arch = "wasm32")]
// impl From<Style> for AtValue {
//     fn from(item: Style) -> Self {
//         AtValue::Some(item.class_name.clone())
//     }
// }

// this is going to be needed for Seed v0.7.0
// #[cfg(target_arch = "wasm32")]
// use seed::virtual_dom::ToClasses;

// #[cfg(target_arch = "wasm32")]
// impl ToClasses for Style {
//     fn to_classes(self) -> Option<Vec<String>> {
//         let mut classes = Vec::new();
//         classes.push(self.class_name.clone());
//         Some(classes)
//     }
// }
