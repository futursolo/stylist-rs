use super::super::css_ident::CssIdent;
use proc_macro2::{Literal, Punct, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug, Clone)]
pub enum PreservedToken {
    Punct(Punct),
    Literal(Literal),
    Ident(CssIdent),
}

impl ToTokens for PreservedToken {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Self::Ident(i) => i.to_tokens(toks),
            Self::Literal(i) => i.to_tokens(toks),
            Self::Punct(i) => i.to_tokens(toks),
        }
    }
}

impl Parse for PreservedToken {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        if CssIdent::peek(input) {
            Ok(Self::Ident(input.parse()?))
        } else if input.cursor().punct().is_some() {
            Ok(Self::Punct(input.parse()?))
        } else if input.cursor().literal().is_some() {
            Ok(Self::Literal(input.parse()?))
        } else {
            Err(input.error("Expected a css identifier, punctuation or a literal"))
        }
    }
}

impl PreservedToken {
    pub fn to_output_string(&self) -> String {
        match self {
            Self::Ident(i) => i.to_output_string(),
            Self::Literal(l) => format!("{l}"),
            Self::Punct(p) => format!("{}", p.as_char()),
        }
    }
}
