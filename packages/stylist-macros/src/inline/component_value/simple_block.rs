use super::ComponentValue;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseBuffer, Result as ParseResult},
    token,
};

#[derive(Debug, Clone)]
pub enum SimpleBlock {
    Braced {
        brace: token::Brace,
        contents: Vec<ComponentValue>,
    },
    Bracketed {
        bracket: token::Bracket,
        contents: Vec<ComponentValue>,
    },
    Paren {
        paren: token::Paren,
        contents: Vec<ComponentValue>,
    },
}

impl ToTokens for SimpleBlock {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Self::Braced { brace, contents } => brace.surround(toks, |toks| {
                for c in contents.iter() {
                    c.to_tokens(toks);
                }
            }),
            Self::Bracketed { bracket, contents } => bracket.surround(toks, |toks| {
                for c in contents.iter() {
                    c.to_tokens(toks);
                }
            }),
            Self::Paren { paren, contents } => paren.surround(toks, |toks| {
                for c in contents.iter() {
                    c.to_tokens(toks);
                }
            }),
        }
    }
}

impl Parse for SimpleBlock {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace) {
            let inside;
            let brace = braced!(inside in input);
            let contents = ComponentValue::parse_multiple(&inside)?;
            Ok(Self::Braced { brace, contents })
        } else if lookahead.peek(token::Bracket) {
            let inside;
            let bracket = bracketed!(inside in input);
            let contents = ComponentValue::parse_multiple(&inside)?;
            Ok(Self::Bracketed { bracket, contents })
        } else if lookahead.peek(token::Paren) {
            let inside;
            let paren = parenthesized!(inside in input);
            let contents = ComponentValue::parse_multiple(&inside)?;
            Ok(Self::Paren { paren, contents })
        } else {
            Err(lookahead.error())
        }
    }
}
