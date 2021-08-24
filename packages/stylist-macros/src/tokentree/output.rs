use crate::tokentree::component_value::ComponentValue;
use crate::tokentree::component_value::PreservedToken;
use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
// =====================================================================
// =====================================================================
// Output structs + quoting
// =====================================================================
// =====================================================================
pub struct OutputSheet {
    pub contents: Vec<TokenStream>,
}
pub struct OutputAtRule {
    pub name: TokenStream,
    pub prelude: Vec<TokenStream>,
    pub contents: Vec<TokenStream>,
}
pub struct OutputQualifiedRule {
    pub qualifier: TokenStream,
    pub attributes: Vec<TokenStream>,
}
pub enum OutputScopeContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}
pub enum OutputRuleContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}
#[derive(Clone)]
pub struct OutputQualifier {
    pub selectors: Vec<ComponentValue>,
}
pub struct OutputAttribute {
    pub key: TokenStream,
    pub values: Vec<TokenStream>,
}

/// Reify a structure into an expression of a specific type.
pub(crate) trait Reify {
    fn reify(self) -> TokenStream;
}

impl Reify for OutputSheet {
    fn reify(self) -> TokenStream {
        let ident_scopes = Ident::new("scopes", Span::mixed_site());
        let Self { contents } = self;

        quote! {
            ::stylist::ast::SheetRef::from(
                ::stylist::ast::Sheet::from(
                    {
                        let mut #ident_scopes = ::std::vec::Vec::<::stylist::ast::ScopeContent>::new();
                        #( #ident_scopes.push( #contents ); )*
                        #ident_scopes
                    }
                )
            )
        }
    }
}

impl Reify for OutputAtRule {
    fn reify(self) -> TokenStream {
        let ident_condition = Ident::new("at_condition", Span::mixed_site());
        let ident_attributes = Ident::new("attributes", Span::mixed_site());
        let Self {
            name,
            prelude,
            contents,
        } = self;

        quote! {
            ::stylist::ast::Rule {
                condition: {
                    let mut #ident_condition = ::std::vec::Vec::<::stylist::ast::StringFragment>::new();
                    #ident_condition.push( "@".into() );
                    #ident_condition.push( #name );
                    #ident_condition.push( " ".into() );
                    #( #ident_condition.push(#prelude); )*
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

impl Reify for OutputQualifiedRule {
    fn reify(self) -> TokenStream {
        let ident_attributes = Ident::new("attributes", Span::mixed_site());
        let Self {
            qualifier,
            attributes,
            ..
        } = self;

        quote! {
            ::stylist::ast::Block {
                condition: #qualifier,
                style_attributes: {
                    let mut #ident_attributes = ::std::vec::Vec::<::stylist::ast::StyleAttribute>::new();
                    #( #ident_attributes.push(#attributes); )*
                    #ident_attributes.into()
                },
            }
        }
    }
}

impl Reify for OutputQualifier {
    fn reify(self) -> TokenStream {
        fn is_not_comma(q: &ComponentValue) -> bool {
            !matches!(q, ComponentValue::Token(PreservedToken::Punct(ref p)) if p.as_char() == ',')
        }

        let ident_selector = Ident::new("conditions", Span::mixed_site());
        let Self { selectors, .. } = self;
        let selectors = selectors
            .iter()
            .peekable()
            .batching(|it| {
                it.peek()?; // Return if no items left
                let ident_selector = Ident::new("selector", Span::mixed_site());
                let ident_assert_string = Ident::new("as_string", Span::mixed_site());
                // Take until the next comma
                let selector_parts = it
                    .peeking_take_while(|q| is_not_comma(q))
                    .flat_map(|p| p.reify());
                let selector = quote! {
                    {
                        use ::std::fmt::Write;
                        fn #ident_assert_string(s: ::std::string::String) -> ::std::string::String { s }
                        let mut #ident_selector = ::std::string::String::new();
                        #( ::std::write!(&mut #ident_selector, "{}", #ident_assert_string(#selector_parts)).expect(""); )*
                        ::stylist_core::ast::Selector::from(#ident_selector)
                    }
                };
                it.next(); // Consume the comma
                Some(selector)
            });
        quote! {
            {
                let mut #ident_selector = ::std::vec::Vec::<::stylist::ast::Selector>::new();
                #( #ident_selector.push(#selectors); )*
                #ident_selector.into()
            }
        }
    }
}

impl Reify for OutputScopeContent {
    fn reify(self) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.reify();
                quote! { ::stylist::ast::ScopeContent::Rule(#block_tokens) }
            }
            Self::Block(block) => {
                let block_tokens = block.reify();
                quote! { ::stylist::ast::ScopeContent::Block(#block_tokens) }
            }
        }
    }
}

impl Reify for OutputRuleContent {
    fn reify(self) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.reify();
                quote! { ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#block_tokens)) }
            }
            Self::Block(block) => {
                let block_tokens = block.reify();
                quote! { ::stylist::ast::RuleContent::Block(#block_tokens) }
            }
        }
    }
}

impl Reify for OutputAttribute {
    fn reify(self) -> TokenStream {
        let ident_writable_value = Ident::new("writer_value", Span::mixed_site());
        let Self { key, values } = self;

        quote! {
            ::stylist::ast::StyleAttribute {
                key: #key,
                value: {
                    let mut #ident_writable_value = ::std::vec::Vec::<::stylist::ast::StringFragment>::new();
                    #( #ident_writable_value.push(#values); )*
                    #ident_writable_value.into()
                },
            }
        }
    }
}

impl<E: Reify> Reify for Result<E, TokenStream> {
    fn reify(self) -> TokenStream {
        match self {
            Ok(o) => o.reify(),
            Err(e) => e,
        }
    }
}
