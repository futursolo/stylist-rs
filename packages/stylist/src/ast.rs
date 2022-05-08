//! This module contains the semantic representation of a CSS StyleSheet.
//!
//! ```text
//! Sheet
//! └── Vec<enum ScopeContent>
//!     ├── Block
//!     │   ├── condition: Vec<Selector>
//!     │   │   └── fragments: Vec<StringFragment>
//!     │   └── content: Vec<enum RuleBlockContent>
//!     │       ├── StyleAttr
//!     │       │   ├── key: String
//!     │       │   └── value: Vec<StringFragment>
//!     │       ├── Block (*)
//!     │       └── Rule (*)
//!     └── Rule
//!         ├── condition: Vec<StringFragment>
//!         └── Vec<enum RuleBlockContent (*)>
//! ```
//!
//! # Warning
//!
//! This module is not stable at the moment and is exposed to be used by procedural macros.
//! Its API may change at anytime.

/// A procedural macro that parses a string literal or an inline stylesheet into a [`Sheet`].
///
/// Please consult the documentation of the [`macros`](crate::macros) module for the supported
/// syntax of this macro.
///
/// # Warning
///
/// Use of this macro is discouraged.
///
/// Any place that accepts the output of this macro also accepts the output of
/// [`css!`](crate::css).
///
/// Use [`css!`](crate::css) unless you know what you are doing.
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::sheet;

#[doc(inline)]
pub use stylist_core::ast::*;

#[doc(inline)]
pub use stylist_core::bow::Bow;
