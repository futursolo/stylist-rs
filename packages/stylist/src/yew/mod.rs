//! This module contains yew specific features.

use yew::html::Classes;
use yew::html::IntoPropValue;

use crate::ast::SheetRef;
use crate::IntoStyle;
use crate::Style;

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser;
mod global;

pub use global::{Global, GlobalProps};

impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

impl From<IntoStyle> for Classes {
    fn from(into_style: IntoStyle) -> Self {
        let mut classes = Self::new();
        classes.push(into_style.to_style().get_class_name().to_string());
        classes
    }
}

impl IntoPropValue<IntoStyle> for SheetRef {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::Sheet(self)
    }
}
