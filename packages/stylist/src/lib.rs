#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

//! Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.
//!
//! ## Usage
//!
//! ### Procedural Macros
//!
//! To create a stylesheet, you can use [`style!`]:
//!
//! ```
//! use stylist::style;
//!
//! let style = style!(
//!     r#"
//!         background-color: red;
//!
//!         .nested {
//!             background-color: blue;
//!             width: 100px
//!         }
//!     "#
//! );
//! ```
//!
//! ### Style API
//!
//! If you want to parse a string into a style at runtime, you can use [`Style::new`]:
//!
//! ```rust
//! use stylist::Style;
//!
//! let style = Style::new(
//!     r#"
//!         background-color: red;
//!
//!         .nested {
//!             background-color: blue;
//!             width: 100px
//!         }
//!     "#,
//! ).expect("Failed to create style");
//! ```
//!
//! ### YieldStyle API
//!
//! Alternatively, any struct that implements [`YieldStyle`] trait can call
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
//!
//! ```
//!
//! Everything that is not in a conditioned block will be applied to the Component
//! the class of this style is applied to.
//!
//! You may also use Current Selector (`&`) in CSS selectors to denote the container element:
//!
//! ```css
//! &:hover {
//!   background-color: #d0d0d9;
//! }
//! ```
//!
//! You can also use other CSS rules(such as: keyframes, supports and media):
//!
//! ```css
//! @keyframes mymove {
//!   from {
//!     top: 0px;
//!   }
//!   to {
//!     top: 200px;
//!   }
//! }
//! ```
//!
//! ```css
//! @media only screen and (max-width: 600px) {
//!   background-color: #303040;
//!
//!   .nested {
//!     background-color: lightblue;
//!   }
//!
//!   &:hover {
//!     background-color: #606072;
//!   }
//! }
//! ```
//!
//! ```css
//! @supports (backdrop-filter: blur(5px)) {
//!   backdrop-filter: blur(5px);
//! }
//! ```
//!
//! ## Yew Integration
//!
//! To enable yew integration. Enable feature `yew_integration` in `Cargo.toml`.
//!
//! Then create a style and use it with yew like this:
//!
//! ```rust
//! use std::borrow::Cow;
//!
//! use yew::prelude::*;
//! use stylist::css;
//!
//! struct MyStyledComponent {}
//!
//! impl Component for MyStyledComponent {
//!     type Message = ();
//!     type Properties = ();
//!
//!     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//!         Self {}
//!     }
//!
//!     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//!         false
//!     }
//!
//!     fn update(&mut self, _: Self::Message) -> ShouldRender {
//!         false
//!     }
//!
//!     fn view(&self) -> Html {
//!         html! {<div class=css!("color: red;")>{"Hello World!"}</div>}
//!     }
//! }
//! ```
//!
//! ### Theming
//!
//! There're theming examples using
//! [Yewdux](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-yewdux)
//! and [yewtil::store](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-agent).
//!
//! ## Features Flags
//!
//! - `macros`: Enabled by default, this flag enables procedural macro support.
//! - `random`: Enabled by default, this flag uses `rand` crate to generate a random
//!   class name. Disabling this flag will opt for a class name that is counter-based.
//! - `yew_integration`: This flag enables yew integration, which implements [`Classes`](::yew::html::Classes) for
//!   [`Style`] and provides a [`Global`](yew::Global) component for applying global styles.

#[cfg(target_arch = "wasm32")]
#[path = "arch_wasm.rs"]
mod arch;

pub mod manager;
mod registry;

pub mod ast;
mod global_style;
mod style;
mod style_src;
mod utils;
mod yield_style;

pub use global_style::GlobalStyle;
pub use style::Style;
pub use style_src::StyleSource;
pub use yield_style::YieldStyle;

/// A procedural macro that parses a string literal into a [`Style`].
///
/// This macro supports string interpolation, please see documentation of [`css!`] macro for
/// usage.
///
/// # Example
///
/// ```
/// use stylist::style;
///
/// // Returns a Style instance.
/// let style = style!("color: red;");
/// ```
#[doc(inline)]
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::style;

/// A procedural macro that parses a string literal into a [`GlobalStyle`].
///
/// This macro supports string interpolation, please see documentation of [`css!`] macro for
/// usage.
///
/// # Example
///
/// ```
/// use stylist::global_style;
///
/// // Returns a GlobalStyle instance.
/// let style = global_style!("color: red;");
/// ```
#[doc(inline)]
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::global_style;

/// A procedural macro that parses a string literal into a [`StyleSource`].
///
/// # Example
///
/// ```
/// use stylist::css;
/// use stylist::yew::Global;
/// use yew::prelude::*;
///
/// let rendered = html! {<div class=css!("color: red;") />};
/// let rendered_global = html! {<Global css=css!("color: red;") />};
/// ```
///
/// # String Interpolation
///
/// This macro supports string interpolation on values of style attributes, selectors, `@supports` and `@media` rules.
///
/// Interpolated strings are denoted with `${ident}` and any type that implements [`std::fmt::Display`] can be
/// used as value. Only named argument are supported at this moment.
///
/// Stylist currently does not check or escape the content of interpolated strings. It is possible
/// to pass invalid strings that would result in an invalid stylesheet. In debug mode, if feature `parser` is
/// enabled, Stylist will attempt to parse the stylesheet again after interpolated strings are
/// substituted with its actual value to check if the final stylesheet is invalid.
///
/// # Example
///
/// ```
/// use stylist::{Style, css};
/// use yew::prelude::*;
///
/// let s = css!(
///     r#"
///         color: ${color};
///
///         span, ${sel_div} {
///             background-color: blue;
///         }
///
///         @media screen and ${breakpoint} {
///             display: flex;
///         }
///     "#,
///     color = "red",
///     sel_div = "div.selected",
///     breakpoint = "(max-width: 500px)",
/// );
///
/// let style = Style::new(s).expect("Failed to create style");
///
/// // Example Output:
/// // .stylist-fIEWv6EP {
/// // color: red;
/// // }
/// // stylist-fIEWv6EP span, .stylist-fIEWv6EP div.selected {
/// // background-color: blue;
/// // }
/// // @media screen and (max-width: 500px) {
/// // .stylist-fIEWv6EP {
/// // display: flex;
/// // }
/// // }
/// println!("{}", style.get_style_str());
/// ```
#[doc(inline)]
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::css;

#[cfg_attr(documenting, doc(cfg(feature = "yew_integration")))]
#[cfg(feature = "yew_integration")]
pub mod yew;

#[doc(inline)]
pub use stylist_core::{Error, Result};

#[doc(hidden)]
pub mod vendor;
