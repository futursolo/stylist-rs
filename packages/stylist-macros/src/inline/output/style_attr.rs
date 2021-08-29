use super::{super::component_value::ComponentValue, fragment_coalesce, fragment_spacing, Reify};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Result as ParseResult;

pub struct OutputAttribute {
    pub key: TokenStream,
    pub values: Vec<ParseResult<ComponentValue>>,
}

impl Reify for OutputAttribute {
    fn into_token_stream(self) -> TokenStream {
        let Self { key, values } = self;

        let value_parts = values
            .iter()
            .flat_map(|p| match p {
                Err(e) => vec![e.to_compile_error().into()],
                Ok(c) => c.reify_parts().into_iter().collect(),
            })
            .spaced_with(fragment_spacing)
            .coalesce(fragment_coalesce)
            .map(|e| e.into_token_stream());
        quote! {
            ::stylist::ast::StyleAttribute {
                key: #key,
                value: ::std::vec![
                    #( #value_parts, )*
                ].into(),
            }
        }
    }
}
