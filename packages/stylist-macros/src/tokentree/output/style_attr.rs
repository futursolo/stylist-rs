use super::{
    super::{component_value::ComponentValue, spacing_iterator::SpacedIterator},
    fragment_spacing, Reify,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Result as ParseResult, Ident};

pub struct OutputAttribute {
    pub key: TokenStream,
    pub values: Vec<ParseResult<ComponentValue>>,
}

impl Reify for OutputAttribute {
    fn reify(self) -> TokenStream {
        let ident_writable_value = Ident::new("writer_value", Span::mixed_site());
        let Self { key, values } = self;

        let value_parts = values
            .iter()
            .flat_map(|p| match p {
                Err(e) => vec![e.to_compile_error().into()],
                Ok(c) => c.reify_parts().into_iter().collect(),
            })
            .spaced_with(fragment_spacing)
            .map(|e| e.reify());
        quote! {
            ::stylist::ast::StyleAttribute {
                key: #key,
                value: {
                    let mut #ident_writable_value = ::std::vec::Vec::<::stylist::ast::StringFragment>::new();
                    #( #ident_writable_value.push(#value_parts); )*
                    #ident_writable_value.into()
                },
            }
        }
    }
}
