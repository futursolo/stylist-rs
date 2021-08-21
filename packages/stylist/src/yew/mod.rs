//! This module contains yew specific features.

use std::borrow::Cow;

use yew::html::Classes;
use yew::html::IntoPropValue;

use crate::ast::Sheet;
use crate::IntoStyle;
use crate::Style;

mod global;

pub use global::{Global, GlobalProps};

impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

impl IntoPropValue<Style> for String {
    fn into_prop_value(self) -> Style {
        self.parse().expect("Failed to parse style.")
    }
}

impl IntoPropValue<Style> for &str {
    fn into_prop_value(self) -> Style {
        self.parse().expect("Failed to parse style.")
    }
}

impl IntoPropValue<Style> for Cow<'_, str> {
    fn into_prop_value(self) -> Style {
        self.parse().expect("Failed to parse style.")
    }
}

impl From<IntoStyle> for Classes {
    fn from(into_style: IntoStyle) -> Self {
        let mut classes = Self::new();
        classes.push(into_style.to_style().get_class_name().to_string());
        classes
    }
}

impl IntoPropValue<IntoStyle> for String {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::String(self.into())
    }
}

impl IntoPropValue<IntoStyle> for &'static str {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::String(self.into())
    }
}

impl IntoPropValue<IntoStyle> for Cow<'static, str> {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::String(self.into())
    }
}

impl IntoPropValue<IntoStyle> for Sheet {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::Sheet(Cow::Owned(self))
    }
}

impl IntoPropValue<IntoStyle> for &'static Sheet {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::Sheet(Cow::Borrowed(self))
    }
}

impl IntoPropValue<IntoStyle> for Cow<'static, Sheet> {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::Sheet(self)
    }
}
