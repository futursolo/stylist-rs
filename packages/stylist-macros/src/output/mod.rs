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
pub use selector::{OutputQualifier, OutputSelector};
mod scope_content;
pub use scope_content::OutputScopeContent;
mod rule_content;
pub use rule_content::OutputRuleContent;
mod style_attr;
pub use style_attr::OutputAttribute;
mod cow_str;
pub use cow_str::OutputCowString;
mod str_frag;
pub use str_frag::{fragment_coalesce, OutputFragment};

mod context_recorder;
pub use context_recorder::ContextRecorder;
mod maybe_static;
pub use maybe_static::IntoCowVecTokens;

/// Reify a structure into an expression of a specific type.
pub trait Reify {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream;
}

impl Reify for syn::Error {
    fn into_token_stream(self, _ctx: &mut ContextRecorder) -> TokenStream {
        self.into_compile_error()
    }
}
