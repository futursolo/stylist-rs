use super::{
    super::{component_value::ComponentValue, spacing_iterator::SpacedIterator},
    fragment_spacing, Reify,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Error as ParseError, Ident};

pub struct OutputAtRule {
    pub name: TokenStream,
    pub prelude: Vec<ComponentValue>,
    pub contents: Vec<TokenStream>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAtRule {
    fn reify(self) -> TokenStream {
        let ident_condition = Ident::new("at_condition", Span::mixed_site());
        let ident_attributes = Ident::new("attributes", Span::mixed_site());
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
            .map(|e| e.reify());
        let errors = errors.into_iter().map(|e| e.into_compile_error());
        quote! {
            ::stylist::ast::Rule {
                condition: {
                    #( #errors )*
                    let mut #ident_condition = ::std::vec::Vec::<::stylist::ast::StringFragment>::new();
                    #ident_condition.push( "@".into() );
                    #ident_condition.push( #name );
                    #ident_condition.push( " ".into() );
                    #( #ident_condition.push(#prelude_parts); )*
                    #ident_condition.into()
                },
                content: {
                    let mut #ident_attributes = ::std::vec::Vec::<::stylist::ast::RuleContent>::new();
                    #( #ident_attributes.push(#contents); )*
                    #ident_attributes.into()
                },
            }
        }
    }
}
