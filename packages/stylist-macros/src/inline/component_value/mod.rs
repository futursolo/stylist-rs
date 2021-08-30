//! The most prominent token in the css spec is called "component values".
//! You can think of this as being either a block, a function or a preserved (atomic) token.
//!
//! This guides our inline parser as follows:
//! - first re-tokenize the TokenStream into a stream of ComponentValues. For this step see
//!   also [`ComponentValueStream`].
//! - parse and verify the component values into blocks, @-rules and attributes.
//!
//! In general, only a parse error in the first step should be fatal and panic immediately,
//! while a parse error in the second step can recover and display a small precise error location
//! to the user, then continue parsing the rest of the input.
use super::{css_ident::CssIdent, output::OutputFragment};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token, Lit,
};

mod preserved_token;
pub use preserved_token::PreservedToken;
mod simple_block;
pub use simple_block::SimpleBlock;
mod function_token;
pub use function_token::FunctionToken;
mod stream;
pub use stream::ComponentValueStream;
mod interpolated_expression;
pub use interpolated_expression::InterpolatedExpression;

#[derive(Debug, Clone)]
pub enum ComponentValue {
    Function(FunctionToken),
    Token(PreservedToken),
    Block(SimpleBlock),
    Expr(InterpolatedExpression),
}

impl ToTokens for ComponentValue {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Self::Block(b) => b.to_tokens(toks),
            Self::Function(f) => f.to_tokens(toks),
            Self::Token(t) => t.to_tokens(toks),
            Self::Expr(e) => e.to_tokens(toks),
        }
    }
}

impl Parse for ComponentValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let is_group =
            input.peek(token::Brace) || input.peek(token::Bracket) || input.peek(token::Paren);
        if is_group {
            Ok(Self::Block(input.parse()?))
        } else if input.peek(token::Dollar) && input.peek2(token::Brace) {
            Ok(Self::Expr(input.parse()?))
        } else if !CssIdent::peek(input) {
            Ok(Self::Token(input.parse()?))
        } else {
            let ident = input.parse()?;
            if input.peek(token::Paren) {
                Ok(Self::Function(FunctionToken::parse_with_name(
                    ident, input,
                )?))
            } else {
                Ok(Self::Token(PreservedToken::Ident(ident)))
            }
        }
    }
}

impl ComponentValue {
    fn parse_multiple(input: &ParseBuffer) -> ParseResult<Vec<Self>> {
        ComponentValueStream::from(input).collect()
    }
}

impl ComponentValue {
    pub fn to_output_fragments(&self) -> impl '_ + IntoIterator<Item = OutputFragment> {
        use std::iter::once;
        match self {
            Self::Token(token) => {
                Box::new(once(token.clone().into())) as Box<dyn Iterator<Item = _>>
            }
            Self::Expr(expr) => Box::new(once(expr.to_output_fragment())),
            Self::Block(SimpleBlock::Bracketed { contents, .. }) => {
                let inner_parts = contents.iter().flat_map(|c| c.to_output_fragments());
                Box::new(once('['.into()).chain(inner_parts).chain(once(']'.into())))
            }
            Self::Block(SimpleBlock::Paren { contents, .. }) => {
                let inner_parts = contents.iter().flat_map(|c| c.to_output_fragments());
                Box::new(once('('.into()).chain(inner_parts).chain(once(')'.into())))
            }
            Self::Function(FunctionToken { name, args, .. }) => {
                let inner_args = args.iter().flat_map(|arg| arg.to_output_fragments());
                Box::new(
                    once(name.clone().into())
                        .chain(once('('.into()))
                        .chain(inner_args)
                        .chain(once(')'.into())),
                )
            }
            Self::Block(SimpleBlock::Braced { .. }) => {
                // this kind of block is not supposed to appear in @-rule preludes, block qualifiers
                // or attribute values and as such should not get emitted
                unreachable!("braced blocks should not get reified");
            }
        }
    }

    // Overly simplified parsing of a css attribute
    #[must_use = "validation errors should not be discarded"]
    pub fn validate_attribute_token(&self) -> Vec<ParseError> {
        match self {
            Self::Expr(_)
            | Self::Token(PreservedToken::Ident(_))
            | Self::Token(PreservedToken::Literal(_)) => vec![],
            Self::Function(FunctionToken { args, .. }) => args
                .iter()
                .flat_map(|a| a.validate_attribute_token())
                .collect(),
            Self::Block(_) => {
                let error = ParseError::new_spanned(
                    self,
                    concat!(
                        "expected a valid part of an attribute, got a block. ",
                        "Did you mean to write `${..}` to inject an expression?"
                    ),
                );
                vec![error]
            }
            Self::Token(PreservedToken::Punct(p)) => {
                if !"-/%:,#".contains(p.as_char()) {
                    vec![ParseError::new_spanned(
                        self,
                        "expected a valid part of an attribute",
                    )]
                } else {
                    vec![]
                }
            }
        }
    }

    // Overly simplified version of parsing a css selector :)
    pub fn validate_selector_token(&self) -> ParseResult<Vec<ParseError>> {
        match self {
            Self::Expr(_) | Self::Function(_) | Self::Token(PreservedToken::Ident(_)) => Ok(vec![]),
            Self::Block(SimpleBlock::Bracketed { contents, .. }) => {
                let mut collected = vec![];
                for e in contents.iter().map(|e| e.validate_selector_token()) {
                    collected.extend(e?);
                }
                Ok(collected)
            }
            Self::Block(_) => Ok(vec![ParseError::new_spanned(
                self,
                "expected a valid part of a scope qualifier, not a block",
            )]),
            Self::Token(PreservedToken::Literal(l)) => {
                let syn_lit = Lit::new(l.clone());
                if !matches!(syn_lit, Lit::Str(_)) {
                    Ok(vec![ParseError::new_spanned(
                        self,
                        "only string literals are allowed in selectors",
                    )])
                } else {
                    Ok(vec![])
                }
            }
            Self::Token(PreservedToken::Punct(p)) => {
                if p.as_char() == ';' {
                    Err(ParseError::new_spanned(
                        self,
                        "unexpected ';' in selector, did you mean to write an attribute?",
                    ))
                } else if !"&>+~|$*=^#.:,".contains(p.as_char()) {
                    Ok(vec![ParseError::new_spanned(
                        self,
                        "unexpected punctuation in selector",
                    )])
                } else {
                    Ok(vec![])
                }
            }
        }
    }
}
