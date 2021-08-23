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
    pub selectors: Vec<TokenStream>,
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
        let Self { contents } = self;
        let ident_scopes = Ident::new("scopes", Span::mixed_site());

        quote! {
            {
                let mut #ident_scopes = ::std::vec::Vec::<::stylist::ast::ScopeContent>::new();
                #( #ident_scopes.push( #contents ); )*
                ::stylist::ast::SheetRef::from(::stylist::ast::Sheet::from(#ident_scopes))
            }
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
            {
                let mut #ident_condition = ::std::vec::Vec::<::stylist::ast::StringFragment>::new();
                #ident_condition.push( "@".into() );
                #ident_condition.push( #name );
                #ident_condition.push( " ".into() );
                #( #ident_condition.push(#prelude); )*

                let mut #ident_attributes = ::std::vec::Vec::<::stylist::ast::RuleContent>::new();
                #( #ident_attributes.push(#contents); )*

                ::stylist::ast::Rule {
                    condition: #ident_condition.into(),
                    content: #ident_attributes.into(),
                }
            }
        }
    }
}

impl Reify for OutputQualifiedRule {
    fn reify(self) -> TokenStream {
        let conditions = self.qualifier;
        let attributes = self.attributes;
        let ident_attributes = Ident::new("attributes", Span::mixed_site());

        quote! {
            {
                let mut #ident_attributes = ::std::vec::Vec::<::stylist::ast::StyleAttribute>::new();
                #( #ident_attributes.push(#attributes); )*

                ::stylist::ast::Block {
                    condition: #conditions,
                    style_attributes: #ident_attributes.into(),
                }
            }
        }
    }
}

impl Reify for OutputQualifier {
    fn reify(self) -> TokenStream {
        let ident_selector = Ident::new("conditions", Span::mixed_site());
        let selectors = self.selectors;
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
                quote! {
                    ::stylist::ast::ScopeContent::Rule(#block_tokens)
                }
            }
            Self::Block(block) => {
                let block_tokens = block.reify();
                quote! {
                    ::stylist::ast::ScopeContent::Block(#block_tokens)
                }
            }
        }
    }
}

impl Reify for OutputRuleContent {
    fn reify(self) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.reify();
                quote! {
                    ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#block_tokens))
                }
            }
            Self::Block(block) => {
                let block_tokens = block.reify();
                quote! {
                    ::stylist::ast::RuleContent::Block(#block_tokens)
                }
            }
        }
    }
}

impl Reify for OutputAttribute {
    fn reify(self) -> TokenStream {
        let ident_writable_value = Ident::new("writer_value", Span::mixed_site());
        let key_tokens = self.key;
        let value_tokens = self.values;

        quote! {
            ::stylist::ast::StyleAttribute {
                key: { #key_tokens }.into(),
                value: {
                    let mut #ident_writable_value = ::std::vec::Vec::<::stylist::ast::StringFragment>::new();
                    #( #ident_writable_value.push(#value_tokens); )*
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
