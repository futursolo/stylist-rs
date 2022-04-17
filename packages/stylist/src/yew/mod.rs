//! This module contains yew specific features.
//!
//! ## Usage in function components
//!
//! You can create a style and use it like this:
//!
//! ```rust
//! use yew::prelude::*;
//! use stylist::yew::use_style;
//!
//! #[function_component]
//! fn MyStyledComponent() -> Html {
//!     let style = use_style!("color: red;");
//!     html! {<div class={style}>{"Hello World!"}</div>}
//! }
//! ```
//! ### Usage in struct components with the YieldStyle API
//!
//! Any struct that implements [`YieldStyle`] trait can call
//! [`self.style()`](YieldStyle::style) to get a [`Style`] instance.
//!
//! ```rust
//! use std::borrow::Cow;
//! use stylist::{css, StyleSource, YieldStyle};
//!
//! pub struct Component;
//!
//! impl YieldStyle for Component {
//!     fn style_from(&self) -> StyleSource<'static> {
//!         css!("color: red;")
//!     }
//! }
//!
//! impl Component {
//!     fn print_style(&self) {
//!         println!("{}", self.style().get_class_name());
//!     }
//! }
//! ```
//!

use yew::html::{Classes, IntoPropValue};

/// A procedural macro to style a function component.
///
/// Specifically this introduces a specialized [`css!`](crate::css) macro
/// that is aware of the contextual style manager.
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

/// A procedural macro to use a specialized, contextual [`css!`](crate::css) macro.
///
/// [`styled_component`] is implemented in terms of this, prefer that if possible.
/// If you need to use [`function_component`](::yew::function_component) directly
/// but still inject the contextual `css!` macro, use this.
///
/// You can also use the attribute on functions that have access to [Hooks] to enable
/// the usage of a contextual `css!` in their body.
///
/// # Example:
///
/// ```rust
/// use yew::prelude::*;
/// use stylist::yew::styled_component_impl;
///
/// // Equivalent to #[styled_component(MyStyledComponent)]
/// // This usage is discouraged, prefer `styled_component`
/// #[styled_component_impl]
/// #[function_component(MyStyledComponent)]
/// fn my_styled_component() -> Html {
///     html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
/// }
/// ```
///
/// [Hooks]: https://yew.rs/next/concepts/function-components#hooks
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::styled_component_impl;

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
        #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
        classes.extend(style.extra_classes);
        classes
    }
}

impl IntoPropValue<StyleSource> for Sheet {
    fn into_prop_value(self) -> StyleSource {
        self.into()
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use std::borrow::Cow;

    use super::*;
    use stylist_core::ResultDisplay;

    impl IntoPropValue<StyleSource> for String {
        fn into_prop_value(self) -> StyleSource {
            self.try_into()
                .expect_display("couldn't parse style string")
        }
    }

    impl<'a> IntoPropValue<StyleSource> for &'a str {
        fn into_prop_value(self) -> StyleSource {
            self.try_into()
                .expect_display("couldn't parse style string")
        }
    }

    impl<'a> IntoPropValue<StyleSource> for Cow<'a, str> {
        fn into_prop_value(self) -> StyleSource {
            self.try_into()
                .expect_display("couldn't parse style string")
        }
    }
}
