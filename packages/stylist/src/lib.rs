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
//! To enable yew integration. Enable feature `yew_integration` in `Cargo.toml`.
//!
//! You can create a style and use it with yew like this:
//!
//! ```rust
//! use stylist::yew::styled_component;
//! use yew::prelude::*;
//!
//! #[styled_component(MyStyledComponent)]
//! fn my_styled_component() -> Html {
//!     html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
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
//! )
//! .expect("Failed to mount style!");
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
//! )
//! .expect("Failed to create style");
//! ```
//!
//! ### Syntax
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
//! There's theming example using
//! [Yew Context API](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-context).
//!
//! ## Features Flags
//!
//! - `macros`: Enabled by default, this flag enables procedural macro support.
//! - `random`: Enabled by default, this flag uses `fastrand` crate to generate a random class name.
//!   Disabling this flag will opt for a class name that is counter-based.
//! - `yew_integration`: This flag enables yew integration, which implements
//!   [`Classes`](::yew::html::Classes) for [`Style`] and provides a [`Global`](yew::Global)
//!   component for applying global styles.

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
