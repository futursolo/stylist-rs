use super::{
    component_value::{ComponentValue, PreservedToken},
    css_ident::CssIdent,
    spacing_iterator::SpacedIterator,
};
use itertools::Itertools;
use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    Ident, LitStr,
};
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
    pub prelude: Vec<ComponentValue>,
    pub contents: Vec<TokenStream>,
    pub errors: Vec<ParseError>,
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
    pub errors: Vec<ParseError>,
}
pub struct OutputAttribute {
    pub key: TokenStream,
    pub values: Vec<ParseResult<ComponentValue>>,
}
#[derive(Debug)]
pub enum OutputFragment {
    Raw(TokenStream),
    Token(PreservedToken),
    Str(LitStr),
    Delimiter(Delimiter, /*start:*/ bool),
}

impl From<char> for OutputFragment {
    fn from(c: char) -> Self {
        match c {
            '{' => Self::Delimiter(Delimiter::Brace, true),
            '}' => Self::Delimiter(Delimiter::Brace, false),
            '[' => Self::Delimiter(Delimiter::Bracket, true),
            ']' => Self::Delimiter(Delimiter::Bracket, false),
            '(' => Self::Delimiter(Delimiter::Parenthesis, true),
            ')' => Self::Delimiter(Delimiter::Parenthesis, false),
            ' ' => Self::Str(LitStr::new(" ", Span::call_site())),
            _ => unreachable!("Delimiter {} not recognized", c),
        }
    }
}

impl From<TokenStream> for OutputFragment {
    fn from(t: TokenStream) -> Self {
        Self::Raw(t)
    }
}

impl From<PreservedToken> for OutputFragment {
    fn from(t: PreservedToken) -> Self {
        Self::Token(t)
    }
}

impl From<LitStr> for OutputFragment {
    fn from(t: LitStr) -> Self {
        Self::Str(t)
    }
}

impl From<CssIdent> for OutputFragment {
    fn from(i: CssIdent) -> Self {
        PreservedToken::Ident(i).into()
    }
}

/// Reify a structure into an expression of a specific type.
pub(crate) trait Reify {
    fn reify(self) -> TokenStream;
}

impl Reify for TokenStream {
    fn reify(self) -> Self {
        self
    }
}

impl Reify for syn::Error {
    fn reify(self) -> TokenStream {
        self.into_compile_error()
    }
}

impl Reify for OutputSheet {
    fn reify(self) -> TokenStream {
        let ident_scopes = Ident::new("scopes", Span::mixed_site());
        let Self { contents } = self;

        quote! {
            ::stylist::ast::Sheet::from(
                {
                    let mut #ident_scopes = ::std::vec::Vec::<::stylist::ast::ScopeContent>::new();
                    #( #ident_scopes.push( #contents ); )*
                    #ident_scopes
                }
            )
        }
    }
}

fn fragment_spacing(l: &OutputFragment, r: &OutputFragment) -> Option<OutputFragment> {
    use OutputFragment::*;
    use PreservedToken::*;
    match (l, r) {
        (Delimiter(_, false), Token(Ident(_))) => true,
        (Token(Ident(_)) | Token(Literal(_)), Token(Ident(_)) | Token(Literal(_))) => true,
        _ => false,
    }
    .then(|| ' '.into())
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

impl OutputFragment {
    fn str_for_delim(d: Delimiter, start: bool) -> &'static str {
        match (d, start) {
            (Delimiter::Brace, true) => "{",
            (Delimiter::Brace, false) => "}",
            (Delimiter::Bracket, true) => "[",
            (Delimiter::Bracket, false) => "]",
            (Delimiter::Parenthesis, true) => "(",
            (Delimiter::Parenthesis, false) => ")",
            (Delimiter::None, _) => unreachable!("only actual delimiters allowed"),
        }
    }
}

impl Reify for OutputFragment {
    fn reify(self) -> TokenStream {
        match self {
            Self::Raw(t) => t,
            Self::Str(lit) => quote! { #lit.into() },
            Self::Token(t) => Self::from(t.quote_literal()).reify(),
            Self::Delimiter(kind, start) => {
                let lit = LitStr::new(Self::str_for_delim(kind, start), Span::call_site());
                Self::from(lit).reify()
            }
        }
    }
}

impl<E: Reify> Reify for Result<E, syn::Error> {
    fn reify(self) -> TokenStream {
        match self {
            Ok(o) => o.reify(),
            Err(e) => e.to_compile_error(),
        }
    }
}
