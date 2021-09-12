//! This module intentionally mirrors stylist_core::ast in structure and
//! is responsible for transforming finished macro outputs into the TokenStream
//! emitted by the different macros.
use proc_macro2::TokenStream;

mod block;
mod rule;
mod rule_block_content;
mod scope_content;
mod selector;
mod sheet;
mod str_frag;
mod style_attr;

mod context_recorder;
mod maybe_static;

pub use block::OutputBlock;
pub use rule::OutputRule;
pub use rule_block_content::OutputRuleBlockContent;
pub use scope_content::OutputScopeContent;
pub use selector::OutputSelector;
pub use sheet::OutputSheet;
pub use str_frag::{fragment_coalesce, OutputFragment};
pub use style_attr::OutputAttribute;

pub use context_recorder::ContextRecorder;
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
