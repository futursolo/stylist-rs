#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

//! Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.
//!
//! ## Usage
//!
//! ### Yew Integration
//!
//! To enable yew integration. Enable feature `yew_integration` in `Cargo.toml`.
//!
//! You can create a style and use it with yew like this:
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
//! ).expect("Failed to mount style!");
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

#[cfg_attr(documenting, doc(cfg(feature = "yew_integration")))]
#[cfg(feature = "yew_integration")]
pub mod yew;

#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub mod macros;

/// A procedural macro that parses a string literal or an inline stylesheet into a [`Style`].
///
/// Please consult the documentation of the [`macros`] module for the supported syntax of this macro.
///
/// # Example
///
/// ```
/// use stylist::style;
///
/// // Returns a Style instance.
/// let style = style!("color: red;");
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::style;

/// A procedural macro that parses a string literal or an inline stylesheet into a [`GlobalStyle`].
///
/// Please consult the documentation of the [`macros`] module for the supported syntax of this macro.
///
/// # Example
///
/// ```
/// use stylist::global_style;
///
/// // Returns a GlobalStyle instance.
/// let style = global_style!("color: red;");
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::global_style;

/// A procedural macro that parses a string literal or an inline stylesheet into a [`StyleSource`].
///
/// Please consult the documentation of the [`macros`] module for the supported syntax of this macro.
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
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::css;

#[doc(inline)]
pub use stylist_core::{Error, Result};
