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
//! # Note
//!
//! This module is not stable at the moment and is exposed to be used by procedural macros.
//! Its API may change at anytime.

#[doc(inline)]
pub use stylist_core::ast::*;

#[doc(inline)]
pub use stylist_core::bow::Bow;
