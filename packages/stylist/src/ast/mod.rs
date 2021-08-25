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

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser;
mod into_sheet;
mod sheet_ref;

pub use into_sheet::IntoSheet;
pub use sheet_ref::SheetRef;
