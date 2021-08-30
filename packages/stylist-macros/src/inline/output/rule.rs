use super::{
    super::{component_value::ComponentValue, css_ident::CssIdent},
    fragment_coalesce, fragment_spacing, ContextRecorder, IntoCowVecTokens, OutputFragment,
    OutputRuleContent, Reify,
};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::once;
use syn::parse::Error as ParseError;

pub struct OutputAtRule {
    pub name: CssIdent,
    pub prelude: Vec<ComponentValue>,
    pub contents: Vec<OutputRuleContent>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAtRule {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let Self {
            name,
            prelude,
            contents,
            errors,
        } = self;

        let at_name = OutputFragment::Str(format!("@{} ", name.to_output_string()));
        let prelude_parts = prelude
            .into_iter()
            .flat_map(|p| p.to_output_fragments())
            .spaced_with(fragment_spacing)
            .coalesce(fragment_coalesce);
        let condition = once(at_name).chain(prelude_parts).into_cow_vec_tokens(ctx);
        let content = contents.into_cow_vec_tokens(ctx);
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        quote! {
            ::stylist::ast::Rule {
                condition: {
                    #( #errors )*
                    #condition
                },
                content: #content,
            }
        }
    }
}
