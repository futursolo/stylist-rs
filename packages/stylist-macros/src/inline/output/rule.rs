use std::iter::once;

use super::{
    super::component_value::ComponentValue, fragment_coalesce, fragment_spacing, MaybeStatic,
    OutputFragment, Reify,
};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Error as ParseError, LitStr};

pub struct OutputAtRule {
    pub name: String,
    pub prelude: Vec<ComponentValue>,
    pub contents: MaybeStatic<Vec<TokenStream>>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAtRule {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        let Self {
            name,
            prelude,
            contents,
            errors,
        } = self;

        let prelude_parts = prelude
            .iter()
            .flat_map(|p| p.reify_parts())
            .spaced_with(fragment_spacing)
            .coalesce(fragment_coalesce);
        let errors = errors.into_iter().map(|e| e.into_compile_error());
        let (content, static_content) = contents
            .into_cow_vec_tokens(quote! {::stylist::ast::RuleContent})
            .into_value();

        let printed_name = LitStr::new(&format!("@{} ", name), Span::call_site());
        let at_name = OutputFragment::Str(printed_name);
        let (condition, static_condition) = once(at_name)
            .chain(prelude_parts)
            .map(|e| e.into_token_stream())
            .collect::<MaybeStatic<_>>()
            .into_cow_vec_tokens(quote! {::stylist::ast::StringFragment})
            .into_value();

        MaybeStatic::in_context(
            static_content & static_condition,
            quote! {
                ::stylist::ast::Rule {
                    condition: {
                        #( #errors )*
                        #condition
                    },
                    content: #content,
                }
            },
        )
    }
}
