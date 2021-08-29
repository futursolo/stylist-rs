use super::{
    super::component_value::ComponentValue, fragment_coalesce, fragment_spacing, MaybeStatic, Reify,
};
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
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        let Self { key, values } = self;

        let (value_parts, value_context) = values
            .iter()
            .flat_map(|p| match p {
                Err(e) => vec![e.to_compile_error().into()],
                Ok(c) => c.reify_parts().into_iter().collect(),
            })
            .spaced_with(fragment_spacing)
            .coalesce(fragment_coalesce)
            .map(|e| e.into_token_stream())
            .collect::<MaybeStatic<_>>()
            .into_cow_vec_tokens(quote! {::stylist::ast::StringFragment})
            .into_value();
        MaybeStatic::in_context(
            value_context,
            quote! {
                ::stylist::ast::StyleAttribute {
                    key: #key,
                    value: #value_parts,
                }
            },
        )
    }
}
