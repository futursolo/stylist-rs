//! This module contains yew specific features.

use yew::html::Classes;
use yew::html::IntoPropValue;

use crate::ast::SheetRef;
use crate::{Style, StyleSource};

mod global;

pub use global::{Global, GlobalProps};

impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

impl From<StyleSource<'_>> for Classes {
    fn from(style_src: StyleSource<'_>) -> Self {
        let mut classes = Self::new();
        classes.push(style_src.to_style().get_class_name().to_string());
        classes
    }
}

impl IntoPropValue<StyleSource<'static>> for SheetRef {
    fn into_prop_value(self) -> StyleSource<'static> {
        self.into()
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use std::borrow::Cow;

    use super::*;

    impl IntoPropValue<Style> for String {
        fn into_prop_value(self) -> Style {
            Style::new(self).expect("Failed to parse style.")
        }
    }

    impl IntoPropValue<Style> for &str {
        fn into_prop_value(self) -> Style {
            Style::new(self).expect("Failed to parse style.")
        }
    }

    impl IntoPropValue<Style> for Cow<'_, str> {
        fn into_prop_value(self) -> Style {
            Style::new(self).expect("Failed to parse style.")
        }
    }

    impl IntoPropValue<StyleSource<'static>> for String {
        fn into_prop_value(self) -> StyleSource<'static> {
            self.into()
        }
    }

    impl<'a> IntoPropValue<StyleSource<'static>> for &'static str {
        fn into_prop_value(self) -> StyleSource<'static> {
            self.into()
        }
    }

    impl<'a> IntoPropValue<StyleSource<'static>> for Cow<'static, str> {
        fn into_prop_value(self) -> StyleSource<'static> {
            self.into()
        }
    }
}
