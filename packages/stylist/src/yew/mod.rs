//! This module contains yew specific features.

use yew::html::Classes;
use yew::html::IntoPropValue;

/// A procedural macro to style a function component. Specifically this introduces a
/// specialized [`css!`](crate::css) macro that is aware of the contextual style manager.
///
/// For detailed arguments and usage see also the underlying
/// [`function_component`](::yew::function_component) attribute in Yew.
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
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::styled_component;

/// A procedural macro to use the specialized, contextual [`css!`](crate::css) macro
/// accessible to [`styled_component`]s. Use this on functions that have access to
/// [Hooks](https://yew.rs/next/concepts/function-components#hooks).
///
/// # Example:
///
/// ```rust
/// use stylist::{StyleSource, yew::styled_component_base};
///
/// #[styled_component_base]
/// fn use_styles() -> StyleSource<'static> {
///     css!("color: red;")
/// }
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::styled_component_base;

/// A procedural macro hook that parses a string literal or an inline stylesheet to create auto updating [`Style`]s.
///
/// Please consult the documentation of the [`macros`](crate::macros) module for the supported syntax of this macro.
///
/// # Example
///
/// ```
/// use yew::prelude::*;
/// use stylist::yew::use_style;
///
/// #[function_component(Comp)]
/// fn comp() -> Html {
///     // Returns a Style instance.
///     let style = use_style!("color: red;");
///     html!{<div class={style}>{"Hello world!"}</div>}
/// }
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "yew_use_style")))]
#[cfg(feature = "yew_use_style")]
pub use stylist_macros::use_style;

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
