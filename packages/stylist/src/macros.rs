//! Utility macros for writing (global) styles.
//!
//! This module contains also runtime support for the macros and documents their usage. There are two
//! syntaxes available: using [string interpolation] or [tokentree based].
//!
//! # String Interpolation
//!
//! This macro supports string interpolation on values of style attributes, selectors, `@supports` and `@media` rules.
//!
//! Interpolated strings are denoted with `${ident}` and any type that implements [`Display`] can be
//! used as value. Only named argument are supported at this moment.
//!
//! If you do need to output a `${` sequence, you may use `$${` to escape to a `${`.
//!
//!
//! ## Example
//! ```css
//! content: "$${}";
//! ```
//!
//! Will be turned into:
//! ```css
//! content: "${}";
//! ```
//!
//! ## Note: `$${` escape can only present where `${` is valid in the css stylesheet.
//!
//! Stylist currently does not check or escape the content of interpolated strings. It is possible
//! to pass invalid strings that would result in an invalid stylesheet. In debug mode, if feature `parser` is
//! enabled, Stylist will attempt to parse the stylesheet again after interpolated strings are
//! substituted with its actual value to check if the final stylesheet is invalid.
//!
//! # Example
//!
//! ```
//! use stylist::{Style, css};
//! use yew::prelude::*;
//!
//! let s = css!(
//!     r#"
//!         color: ${color};
//!
//!         span, ${sel_div} {
//!             background-color: blue;
//!         }
//!
//!         @media screen and ${breakpoint} {
//!             display: flex;
//!         }
//!     "#,
//!     color = "red",
//!     sel_div = "div.selected",
//!     breakpoint = "(max-width: 500px)",
//! );
//!
//! let style = Style::new(s).expect("Failed to create style");
//!
//! // Example Output:
//! // .stylist-fIEWv6EP {
//! // color: red;
//! // }
//! // stylist-fIEWv6EP span, .stylist-fIEWv6EP div.selected {
//! // background-color: blue;
//! // }
//! // @media screen and (max-width: 500px) {
//! // .stylist-fIEWv6EP {
//! // display: flex;
//! // }
//! // }
//! println!("{}", style.get_style_str());
//! ```
//!
//! # Tokentree Style
//!
//! The other possibility is oriented around reusing the rust tokenizer where possible instead of passing
//! a string literal to the macro. The hope is to have an improved programming experience by more precise
//! error locations and diagnostics.
//!
//! Like in string interpolation syntax, interpolated values are allowed in most places through the `${expr}`
//! syntax. In distinction, the braces contain a rust expression of any type implementing [`Display`] that
//! will be evaluated in the surrounding context.
//!
//! Due to the tokenizer there are some quirks with literals. For example `4em` would be tokenized as a
//! floating point literal with a missing exponent and a suffix of `m`. To work around this issue, use
//! string interpolation as in `${"4em"}`. Similarly, some color hash-tokens like `#44444e` as misinterpreted,
//! use the same workaround here: `${"#44444e"}`.
//!
//! # Example
//!
//! ```
//! use stylist::{Style, css};
//! use yew::prelude::*;
//!
//! let max_width_cuttoff = "500px";
//! let primary_color = "red";
//! let s = css!(
//!     color: ${primary_color};
//!
//!     span, ${"div.selected"} {
//!         background-color: blue;
//!     }
//!
//!     @media screen and (max-width: ${max_width_cuttoff}) {
//!         display: flex;
//!     }
//! );
//!
//! let style = Style::new(s).expect("Failed to create style");
//!
//! // Example Output:
//! // .stylist-fIEWv6EP {
//! // color: red;
//! // }
//! // stylist-fIEWv6EP span, .stylist-fIEWv6EP div.selected {
//! // background-color: blue;
//! // }
//! // @media screen and (max-width: 500px) {
//! // .stylist-fIEWv6EP {
//! // display: flex;
//! // }
//! // }
//! println!("{}", style.get_style_str());
//! ```
//!
//! [string interpolation]: #string-interpolation
//! [tokentree based]: #tokentree-style
//! [`Display`]: std::fmt::Display

/// A procedural macro that parses a string literal into a [`Style`].
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
///
/// [`Style`]: crate::Style
/// [`macros`]: self
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
pub use stylist_macros::style;

/// A procedural macro that parses a string literal into a [`GlobalStyle`].
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
///
/// [`GlobalStyle`]: crate::GlobalStyle
/// [`macros`]: self
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
pub use stylist_macros::global_style;

/// A procedural macro that parses a string literal into a [`StyleSource`].
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
///
/// [`StyleSource`]: crate::StyleSource
/// [`macros`]: self
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
pub use stylist_macros::css;
