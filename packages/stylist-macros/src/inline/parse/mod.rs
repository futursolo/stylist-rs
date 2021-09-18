use proc_macro2::TokenStream;
use syn::parse::Error as ParseError;

use crate::output::OutputFragment;

mod attribute;
mod block;
mod qualifier;
mod root;
mod rule;
mod scope;
mod scope_content;

pub use attribute::{CssAttribute, CssAttributeName, CssAttributeValue};
pub use block::CssQualifiedRule;
pub use qualifier::CssBlockQualifier;
pub use root::CssRootNode;
pub use rule::CssAtRule;
pub use scope::CssScope;
pub use scope_content::CssScopeContent;

#[derive(Debug, Default)]
pub struct IntoOutputContext {
    errors: Vec<ParseError>,
}

impl IntoOutputContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = ParseError>,
    {
        self.errors.extend(errors);
    }

    pub fn push_error(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    pub fn into_compile_errors(self) -> Option<TokenStream> {
        use quote::quote;

        if self.errors.is_empty() {
            None
        } else {
            let tokens: Vec<TokenStream> = self
                .errors
                .into_iter()
                .map(|m| m.into_compile_error())
                .collect();

            Some(quote! {
                {
                    { #( #tokens )* }

                    ::stylist::ast::Sheet::from(Vec::new())
                }
            })
        }
    }
}

pub fn fragment_spacing(l: &OutputFragment, r: &OutputFragment) -> Option<OutputFragment> {
    use super::component_value::PreservedToken::*;
    use OutputFragment::*;
    let left_ends_compound = matches!(l, Delimiter(_, false) | Token(Ident(_)) | Token(Literal(_)))
        || matches!(l, Token(Punct(ref p)) if p.as_char() == '*');
    let right_starts_compound = matches!(r, Token(Ident(_)) | Token(Literal(_)))
        || matches!(r, Token(Punct(ref p)) if "*#".contains(p.as_char()));
    let needs_spacing = left_ends_compound && right_starts_compound;
    needs_spacing.then(|| ' '.into())
}
