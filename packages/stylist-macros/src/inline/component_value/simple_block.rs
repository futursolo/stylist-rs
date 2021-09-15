use super::ComponentValue;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseBuffer, Result as ParseResult},
    token,
};

#[derive(Debug, Clone)]
pub enum BlockKind {
    Braced(token::Brace),
    Bracketed(token::Bracket),
    Paren(token::Paren),
}

impl BlockKind {
    pub fn surround_tokens(&self) -> (char, char) {
        match self {
            Self::Braced(_) => ('{', '}'),
            Self::Bracketed(_) => ('[', ']'),
            Self::Paren(_) => ('(', ')'),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleBlock {
    pub kind: BlockKind,
    pub contents: Vec<ComponentValue>,
}

impl ToTokens for SimpleBlock {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self.kind {
            BlockKind::Braced(ref m) => m.surround(toks, |toks| {
                for c in self.contents.iter() {
                    c.to_tokens(toks);
                }
            }),
            BlockKind::Bracketed(ref m) => m.surround(toks, |toks| {
                for c in self.contents.iter() {
                    c.to_tokens(toks);
                }
            }),
            BlockKind::Paren(ref m) => m.surround(toks, |toks| {
                for c in self.contents.iter() {
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
            Ok(Self {
                kind: BlockKind::Braced(brace),
                contents,
            })
        } else if lookahead.peek(token::Bracket) {
            let inside;
            let bracket = bracketed!(inside in input);
            let contents = ComponentValue::parse_multiple(&inside)?;
            Ok(Self {
                kind: BlockKind::Bracketed(bracket),
                contents,
            })
        } else if lookahead.peek(token::Paren) {
            let inside;
            let paren = parenthesized!(inside in input);
            let contents = ComponentValue::parse_multiple(&inside)?;
            Ok(Self {
                kind: BlockKind::Paren(paren),
                contents,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
