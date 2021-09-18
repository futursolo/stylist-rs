//! This module contains yew specific features.

use yew::html::Classes;
use yew::html::IntoPropValue;

/// A procedural macro to style Yew component.
///
/// # Example:
///
/// ```rust
/// use std::borrow::Cow;
///
/// use yew::prelude::*;
/// use stylist::yew::styled_component;
///
/// #[styled_component(MyStyledComponent)]
/// fn my_styled_component() -> Html {
///     html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
/// }
/// ```
///
/// # Note:
///
/// You don't need to import [`css!`](crate::css) inside of a `styled_component`.
///
/// This macro imports a special version of [`css!`](crate::css) macro that is aware of the current style manager.
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::styled_component;

use crate::ast::Sheet;
use crate::manager::StyleManager;
use crate::{Style, StyleSource};

use yew::html::ImplicitClone;

impl ImplicitClone for StyleManager {}

mod global;
mod hooks;
mod provider;

pub use global::{Global, GlobalProps};
pub use provider::{ManagerProvider, ManagerProviderProps};

pub use hooks::*;

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

impl IntoPropValue<StyleSource<'static>> for Sheet {
    fn into_prop_value(self) -> StyleSource<'static> {
        self.into()
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use std::borrow::Cow;

    use super::*;

    impl IntoPropValue<StyleSource<'static>> for String {
        fn into_prop_value(self) -> StyleSource<'static> {
            self.into()
        }
    }

    impl<'a> IntoPropValue<StyleSource<'a>> for &'a str {
        fn into_prop_value(self) -> StyleSource<'a> {
            self.into()
        }
    }

    impl<'a> IntoPropValue<StyleSource<'a>> for Cow<'a, str> {
        fn into_prop_value(self) -> StyleSource<'a> {
            self.into()
        }
    }
}
