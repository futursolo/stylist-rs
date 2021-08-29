use super::{
    super::component_value::ComponentValue, fragment_coalesce, fragment_spacing, MaybeStatic, Reify,
};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

pub struct OutputAttribute {
    pub key: MaybeStatic<TokenStream>,
    pub values: Vec<ComponentValue>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAttribute {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        let Self {
            key,
            values,
            errors,
        } = self;
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        let (key, key_context) = key.into_value();
        let (value_parts, value_context) = values
            .iter()
            .flat_map(|p| p.to_output_fragments())
            .spaced_with(fragment_spacing)
            .coalesce(fragment_coalesce)
            .map(|e| e.into_token_stream())
            .collect::<MaybeStatic<_>>()
            .into_cow_vec_tokens(quote! {::stylist::ast::StringFragment})
            .into_value();
        MaybeStatic::in_context(
            key_context & value_context,
            quote! {
                ::stylist::ast::StyleAttribute {
                    key: #key,
                    value: {
                        #( #errors )*
                        #value_parts
                    },
                }
            },
        )
    }
}
