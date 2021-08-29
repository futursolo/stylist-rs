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

/// Reify a structure into an expression of a specific type.
pub(crate) trait Reify {
    fn into_token_stream(self) -> TokenStream;
}

impl Reify for TokenStream {
    fn into_token_stream(self) -> Self {
        self
    }
}

impl Reify for syn::Error {
    fn into_token_stream(self) -> TokenStream {
        self.into_compile_error()
    }
}

impl<E: Reify> Reify for Result<E, syn::Error> {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Ok(o) => o.into_token_stream(),
            Err(e) => e.to_compile_error(),
        }
    }
}
