//! This module contains runtime support for the macros and documents their usage.
//!
//! There are two syntaxes available: [string literal] and [inline].
//!
//! # String Literal
//!
//! This syntax is simliar to the [`format!`] macro from the standard library.
//!
//! The first argument of this syntax is a string literal followed by an argument list. This macro
//! will replace `${arg}` with the argument in the argument list when creating the AST.
//!
//! This syntax supports interpolation on values of style attributes, selectors, `@supports` and
//! `@media` rules.
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
//! use stylist::{css, Style};
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
//! //     color: red;
//! // }
//! // stylist-fIEWv6EP span, .stylist-fIEWv6EP div.selected {
//! //     background-color: blue;
//! // }
//! // @media screen and (max-width: 500px) {
//! //     .stylist-fIEWv6EP {
//! //         display: flex;
//! //     }
//! // }
//! println!("{}", style.get_style_str());
//! ```
//!
//! # Inline
//!
//! You may also directly inline a stylesheet in the macro.
//!
//! Like in string interpolation syntax, interpolated values are allowed in most places through the
//! `${expr}` syntax. In distinction, the braces contain a rust expression of any type implementing
//! [`Display`] will be evaluated in the surrounding context.
//!
//! ## Known Limitations
//!
//! ### Dimensions
//!
//! Due to the tokenizer of the Rust complier, there are some quirks with literals. For example,
//! `4em` would be tokenized as a floating point literal with a missing exponent and a suffix of
//! `m`. To work around this issue, use string interpolation as in `${"4em"}`. Similarly, some color
//! hash-tokens like `#44444e` as misinterpreted, use the same workaround here: `${"#44444e"}`.
//!
//! ### Descendant Selectors
//!
//! The stable Rust tokenizer also currently offers no way to inspect whitespace between tokens, as
//! tracked in [the Span inspection API issue](https://github.com/rust-lang/rust/issues/54725). This means that, e.g. the two
//! selectors `.class-a.class-b` and `.class-a .class-b` can not be differentiated. **The macro errs
//! on side of the former input without any spaces.** If you meant to write the latter, use
//! `.class-a *.class-b`.
//!
//! To be more specific, a space is inserted between two tokens `L R` iff (regardless of the space
//! being present in the macro input):
//! - `L` is either a closing bracket `)}]`, an identifier `red`, a literal string `"\e600"` or
//!   number `3px`, or the '*' character.
//! - `R` is either an identifier, a literal string or number, the '*' or '#' character.
//!   Spacing around interpolation is ignored regardless.
//!
//! Be aware that the above is subject to change once the Span API is stabilized. To avoid future
//! rewriting, use spacing in your source code that follows the same rules. Refer to the associated [bug report](https://github.com/futursolo/stylist-rs/issues/41)
//! to discuss this limitation and offer additional suggestions
//!
//! ### Identifier (Edition 2021)
//!
//! In Rust edition 2021, you cannot have an identifier before `#`.
//! In other words, a selector like `.class#id` will no longer compile in Rust edition 2021 (See:
//! [Reserving Syntax](https://doc.rust-lang.org/edition-guide/rust-2021/reserving-syntax.html) in Rust
//! 2021 Edition Guide).
//!
//! Stylist previously treats them as if there is a descendant selector in between (`.class#id` is
//! interpreted as `.class #id`).
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
//! //     color: red;
//! // }
//! // stylist-fIEWv6EP span, .stylist-fIEWv6EP div.selected {
//! //     background-color: blue;
//! // }
//! // @media screen and (max-width: 500px) {
//! //     .stylist-fIEWv6EP {
//! //         display: flex;
//! //     }
//! // }
//! println!("{}", style.get_style_str());
//! ```
//!
//! ## Security Notice
//!
//! Stylist currently does not check or escape the content of interpolated strings. It is possible
//! to pass invalid strings that would result in an invalid stylesheet. In debug mode, if feature
//! `parser` is enabled, Stylist will attempt to parse the stylesheet again after interpolated
//! strings are substituted with its actual value to check if the final stylesheet is valid.
//!
//! [string literal]: #string-literal
//! [inline]: #inline
//! [`Display`]: std::fmt::Display

#[doc(hidden)]
pub mod vendor {
    pub use once_cell;
}
