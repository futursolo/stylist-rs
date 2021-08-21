//! This module contains the semantic representation of a CSS StyleSheet.
//!
//! ```text
//! struct Sheet
//! └── Vec<enum ScopeContent>
//!     ├── struct Block
//!     │   ├── selector: Vec<Selector>
//!     │   └── Vec<struct StyleAttribute>
//!     │       ├── key: String
//!     │       └── value: String
//!     └── struct Rule
//!         ├── condition: String
//!         └── Vec<enum RuleContent>
//!             ├── Block (*)
//!             └── Rule (*)
//! ```
//!

#[doc(inline)]
pub use stylist_core::ast::*;

/// A procedural macro that parses a string literal into a [`Sheet`].
#[doc(inline)]
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::sheet;
