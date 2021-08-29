//! This module intentionally mirrors stylist_core::ast in structure and
//! is responsible for transforming finished macro outputs into the TokenStream
//! emitted by the different macros.
use proc_macro2::TokenStream;

mod sheet;
pub use sheet::OutputSheet;
mod rule;
pub use rule::OutputAtRule;
mod block;
pub use block::OutputQualifiedRule;
mod selector;
pub use selector::OutputQualifier;
mod scope_content;
pub use scope_content::OutputScopeContent;
mod rule_content;
pub use rule_content::OutputRuleContent;
mod style_attr;
pub use style_attr::OutputAttribute;
mod str_frag;
pub use str_frag::{fragment_coalesce, fragment_spacing, OutputFragment};

mod maybe_static;
pub use maybe_static::MaybeStatic;

/// Reify a structure into an expression of a specific type.
pub(crate) trait Reify {
    fn into_token_stream(self) -> MaybeStatic<TokenStream>;
}

impl Reify for syn::Error {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        MaybeStatic::statick(self.into_compile_error())
    }
}
