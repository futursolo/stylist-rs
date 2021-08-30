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

mod context_recorder;
pub use context_recorder::{AllowedUsage, ContextRecorder};
mod maybe_static;
pub use maybe_static::{IntoCowVecTokens, MaybeStatic};

/// Reify a structure into an expression of a specific type.
pub trait Reify {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream;
    fn into_context_aware_tokens(self) -> MaybeStatic<TokenStream>
    where
        Self: Sized,
    {
        let mut ctx = Default::default();
        let value = self.into_token_stream(&mut ctx);
        MaybeStatic::in_context(value, ctx)
    }
}

impl Reify for syn::Error {
    fn into_token_stream(self, _: &mut ContextRecorder) -> TokenStream {
        self.into_compile_error()
    }
}
