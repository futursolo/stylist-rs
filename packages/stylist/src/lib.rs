#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(clippy::all)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(documenting, feature(doc_auto_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

//! Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.
//!
//! ## Usage
//!
//! ### Yew Integration
//!
//! To enable yew integration, enable the `yew_integration` feature in `Cargo.toml`.
//!
//! For a detailed usage with yew, see the [`yew`](crate::yew) module.
//!
//! ### Syntax
//!
//! Every declaration that is not in a qualified block will be applied to the Component
//! the class of this style is applied to.
//!
//! You may also use `&` in CSS selectors to denote the generated class of the container
//! element:
//!
//! ```css
//! &:hover {
//!   background-color: #d0d0d9;
//! }
//! ```
//!
//! You can also use other CSS at-rules (such as: @keyframes, @supports and @media):
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
//! There's theming example using
//! [Yew Context API](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-context).
//!
//! ### Style API
//!
//! If you want to parse a string into a style at runtime, you need to enable the
//! `parser` feature. You can then use [`Style::new`], passing a `str`, `String`
//! or `Cow<'a, str>`:
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
//! )
//! .expect("Failed to create style");
//! ```
//!
//! ## Features Flags
//!
//! - `macros`: Enabled by default, this flag enables procedural macro support.
//! - `random`: Enabled by default, this flag uses `fastrand` crate to generate a random class name.
//!   Disabling this flag will opt for a class name that is counter-based.
//! - `parser`: Disabled by default, this flag enables runtime parsing of styles from strings. You
//!   don't need to enable this to generate styles via the macros.
//! - `yew_integration`: This flag enables yew integration, which implements
//!   [`Classes`](::yew::html::Classes) for [`Style`] and provides a [`Global`](yew::Global)
//!   component for applying global styles.
//! - `debug_style_locations`: Enabled by default, this flag annotates elements with additional
//!   classes to help debugging and finding the source location of styles.
//! - `debug_parser`: Enabled by default, this flag generates additional checks when
//!   `debug_assertions` are enabled.

#[cfg(any(feature = "yew_use_media_query", target_arch = "wasm32"))]
mod arch;
pub mod ast;
mod global_style;
#[cfg(feature = "macros")]
pub mod macros;
pub mod manager;
mod registry;
mod style;
mod style_src;
mod utils;
#[cfg(feature = "yew")]
pub mod yew;

pub use global_style::GlobalStyle;
pub use style::Style;
pub use style_src::StyleSource;
#[doc(inline)]
pub use stylist_core::{Error, Result};
/// A procedural macro that parses a string literal or an inline stylesheet into a
/// [`StyleSource`].
///
/// Please consult the documentation of the [`macros`] module for the supported syntax of this
/// macro.
///
/// # Example
///
/// ```
/// use stylist::css;
/// use stylist::yew::Global;
/// use yew::prelude::*;
///
/// let rendered = html! {<div class={css!("color: red;")} />};
/// let rendered_global = html! {<Global css={css!("color: red;")} />};
/// ```
#[cfg(feature = "macros")]
pub use stylist_macros::css;
/// A procedural macro that parses a string literal or an inline stylesheet into a
/// [`GlobalStyle`].
///
/// Please consult the documentation of the [`macros`] module for the supported syntax of this
/// macro.
///
/// # Example
///
/// ```
/// use stylist::global_style;
///
/// // Returns a GlobalStyle instance.
/// let style = global_style!("color: red;");
/// ```
#[cfg(feature = "macros")]
pub use stylist_macros::global_style;
/// A procedural macro that parses a string literal or an inline stylesheet into a [`Style`].
///
/// Please consult the documentation of the [`macros`] module for the supported syntax of this
/// macro.
///
/// # Example
///
/// ```
/// use stylist::style;
///
/// // Returns a Style instance.
/// let style = style!("color: red;");
/// ```
#[cfg(feature = "macros")]
pub use stylist_macros::style;
pub use vars::CssVariables;

/// A procedural macro to create a style source for mounting css variables.
///
/// # Example
///
/// ```
/// use stylist::yew::Global;
/// use stylist::CssVariables;
/// # use yew::html;
///
/// #[derive(CssVariables)]
/// struct Theme {
///     color: String,
///     background: String,
/// }
///
/// let light_theme = Theme {
///     color: "black".into(),
///     background: "white".into(),
/// };
///
/// let dark_theme = Theme {
///     color: "white".into(),
///     background: "black".into(),
/// };
///
/// // Themes can be converted to style sources and mounted normally:
///
/// html! {
///     <>
///         <Global css={light_theme.to_css_vars_for("html[data-theme='light']")} />
///         <Global css={dark_theme.to_css_vars_for("html[data-theme='dark']")} />
///     </>
/// };
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::CssVariables;

/// A procedural macro to use a css variable.
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::css_var;
