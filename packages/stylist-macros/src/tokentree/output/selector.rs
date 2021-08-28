use super::{
    super::{
        component_value::{ComponentValue, PreservedToken},
        spacing_iterator::SpacedIterator,
    },
    fragment_spacing, Reify,
};
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Error as ParseError, Ident};

#[derive(Clone)]
pub struct OutputQualifier {
    pub selectors: Vec<ComponentValue>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputQualifier {
    fn reify(self) -> TokenStream {
        fn is_not_comma(q: &&ComponentValue) -> bool {
            !matches!(q, ComponentValue::Token(PreservedToken::Punct(ref p)) if p.as_char() == ',')
        }
        // Reify the expression of a Selector from the expressions of its fragments
        fn reify_selector<'c>(
            selector_parts: impl Iterator<Item = &'c ComponentValue>,
        ) -> TokenStream {
            let ident_selector = Ident::new("selector", Span::mixed_site());
            let ident_assert_string = Ident::new("as_string", Span::mixed_site());
            let parts = selector_parts
                .flat_map(|p| p.reify_parts())
                .spaced_with(fragment_spacing)
                .map(|e| e.reify());
            quote! {
                {
                    use ::std::fmt::Write;
                    fn #ident_assert_string(s: ::std::string::String) -> ::std::string::String { s }
                    let mut #ident_selector = ::std::string::String::new();
                    #( ::std::write!(&mut #ident_selector, "{}", #ident_assert_string(#parts)).expect(""); )*
                    ::stylist::ast::Selector::from(#ident_selector)
                }
            }
        }

        let Self {
            selectors, errors, ..
        } = self;
        let selectors = selectors.iter().peekable().batching(|it| {
            // Return if no items left
            it.peek()?;
            // Take until the next comma
            let selector_parts = it.peeking_take_while(is_not_comma);
            let selector = reify_selector(selector_parts);
            it.next(); // Consume the comma
            Some(selector)
        });
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        let ident_selector = Ident::new("conditions", Span::mixed_site());
        quote! {
            {
                let mut #ident_selector = ::std::vec::Vec::<::stylist::ast::Selector>::new();
                #( #errors )*
                #( #ident_selector.push(#selectors); )*
                #ident_selector.into()
            }
        }
    }
}
