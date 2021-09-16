//! This module contains yew specific features.

use yew::html::Classes;
use yew::html::IntoPropValue;

#[doc(hidden)]
#[cfg(feature = "macros")]
pub use stylist_macros::__css_yew_impl;

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

#[doc(hidden)]
#[macro_export]
macro_rules! __use_stylist_item {
    ($mgr:ident, use css as $i:ident) => {
        macro_rules! $i {
            ($args:tt) => {
                $crate::css!($args).with_manager($mgr.clone())
            };
        }
    };
    ($mgr:ident, use style as $i:ident) => {
        macro_rules! $i {
            ($args:tt) => {
                $crate::style!($args).with_manager($mgr.clone())
            };
        }
    };
    ($mgr:ident, use global_style as $i:ident) => {
        macro_rules! $i {
            ($args:tt) => {
                $crate::global_style!($args).with_manager($mgr.clone())
            };
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __use_stylist_item_dispatch {
    ($mgr:ident, use css as $i:ident) => {
        $crate::__use_stylist_item!($mgr, use css as $i)
    };
    ($mgr:ident, use css) => {
        $crate::__use_stylist_item!($mgr, use css as css)
    };
    ($mgr:ident, use style as $i:ident) => {
        $crate::__use_stylist_item!($mgr, use style as $i)
    };
    ($mgr:ident, use style) => {
        $crate::__use_stylist_item!($mgr, use style as style)
    };
    ($mgr:ident, use global_style as $i:ident) => {
        $crate::__use_stylist_item!($mgr, use global_style as $i)
    };
    ($mgr:ident, use global_style) => {
        $crate::__use_stylist_item!($mgr, use global_style as global_style)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __use_stylist {
    ($($l:ident$( as $i:ident)?),+) => {
        let __stylist_style_manager__ =
            ::yew::functional::use_context::<::stylist::manager::StyleManager>()
                .unwrap_or_default();
        $($crate::__use_stylist_item_dispatch!(__stylist_style_manager__, use $l$( as $i)?));*
    };
}

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
pub use __use_stylist as use_stylist;

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
