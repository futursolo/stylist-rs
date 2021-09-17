//! This module contains yew specific features.

use yew::html::Classes;
use yew::html::IntoPropValue;

/// A procedural macro to style a functional Yew component. This introduces context
/// sensitive versions of stylist macros that respect the [`StyleManager`] scoped to
/// the component.
/// The arguments are the comma separated names of the macros you want to use, optionally
/// followed by `as <ident>`.
///
/// # Example:
///
/// ```rust
/// use yew::prelude::*;
/// use stylist::yew::use_stylist;
///
/// #[function_component(MyStyledComponent)]
/// fn my_styled_component() -> Html {
///     use_stylist!(css, style as sstyle, global_style);
///     html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
/// }
/// ```
#[doc(inline)]
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_yew_macros::__use_stylist as use_stylist;

use crate::ast::Sheet;
use crate::manager::StyleManager;
use crate::{Style, StyleSource};

use yew::html::ImplicitClone;

impl ImplicitClone for StyleManager {}

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
