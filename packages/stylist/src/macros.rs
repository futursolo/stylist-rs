//! This module contains runtime support for the macros and documents their usage.
//!
//! There are two syntaxes available: [string literal] and [inline].
//!
//! # String Literal
//!
//! This introduces a syntax that is simliar to the [`format!`] macro from the standard library.
//!
//! The first argument of this syntax is a string literal followed by an argument list. This macro
//! will replace `${arg}` with the argument in the argument list when creating the AST.
//!
//! This syntax supports interpolation on values of style attributes, selectors, `@supports` and `@media` rules.
//!
//! Interpolated strings are denoted with `${ident}` and any type that implements [`Display`] can be
//! used as value. Only named argument are supported at this moment.
//!
//! If you do need to print a `${` sequence, you may use `$${` to escape to a `${`.
//!
//! ### Example
//! ```css
//! content: "$${}";
//! ```
//!
//! Will be turned into:
//! ```css
//! content: "${}";
//! ```
//!
//! ### Note:
//!
//! `$${` escape can only present where `${` is valid in the css stylesheet.
//!
//! ## Example
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
//! # Inline
//!
//! You may also directly inline a stylesheet in the macro.
//!
//! Like in string interpolation syntax, interpolated values are allowed in most places through the `${expr}`
//! syntax. In distinction, the braces contain a rust expression of any type implementing [`Display`]
//! will be evaluated in the surrounding context.
//!
//! Due to the tokenizer of the Rust complier, there are some quirks with literals. For example, `4em` would be
//! tokenized as a floating point literal with a missing exponent and a suffix of `m`. To work around this issue, use
//! string interpolation as in `${"4em"}`. Similarly, some color hash-tokens like `#44444e` as misinterpreted,
//! use the same workaround here: `${"#44444e"}`.
//!
//! ## Note
//!
//! This syntax provides more precise error locations and advanced diagnostics information.
//!
//! ## Example
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
//! ## Security Notice
//!
//! Stylist currently does not check or escape the content of interpolated strings. It is possible
//! to pass invalid strings that would result in an invalid stylesheet. In debug mode, if feature `parser` is
//! enabled, Stylist will attempt to parse the stylesheet again after interpolated strings are
//! substituted with its actual value to check if the final stylesheet is invalid.
//!
//! [string literal]: #string-literal
//! [inline]: #inline
//! [`Display`]: std::fmt::Display

#[doc(hidden)]
pub mod vendor {
    pub use once_cell;
}
