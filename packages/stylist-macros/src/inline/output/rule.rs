use std::iter::once;

use super::{
    super::component_value::ComponentValue, fragment_coalesce, fragment_spacing, OutputFragment,
    Reify,
};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Error as ParseError, LitStr};

pub struct OutputAtRule {
    pub name: String,
    pub prelude: Vec<ComponentValue>,
    pub contents: Vec<TokenStream>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAtRule {
    fn into_token_stream(self) -> TokenStream {
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

        let printed_name = LitStr::new(&format!("@{} ", name), Span::call_site());
        let at_name = OutputFragment::Str(printed_name);
        let condition_parts = once(at_name)
            .chain(prelude_parts)
            .map(|e| e.into_token_stream());
        quote! {
            ::stylist::ast::Rule {
                condition: ::std::vec![
                    #( #errors, )*
                    #( #condition_parts, )*
                ].into(),
                content: ::std::vec![
                    #( #contents, )*
                ].into(),
            }
        }
    }
}
