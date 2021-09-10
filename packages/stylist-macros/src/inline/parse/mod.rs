use proc_macro2::TokenStream;

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
    errors: Vec<syn::parse::Error>,
}

impl IntoOutputContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = syn::parse::Error>,
    {
        self.errors.extend(errors);
    }

    pub fn push_error(&mut self, error: syn::parse::Error) {
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
    let needs_spacing = matches!(
        (l, r),
        (Delimiter(_, false), Token(Ident(_)))
            | (
                Token(Ident(_)) | Token(Literal(_)),
                Token(Ident(_)) | Token(Literal(_))
            )
    );
    needs_spacing.then(|| ' '.into())
}
