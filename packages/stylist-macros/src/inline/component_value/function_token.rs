use super::{super::css_ident::CssIdent, ComponentValue};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{Parse, ParseBuffer, Result as ParseResult},
    token,
};

// Css-syntax parses like
// v----v function token
// foobar( arg1 , arg2 )
//         ---- - ---- ^-- closing bracket
//         arguments
//
// while this parses like
//
// v-------------------v function token
// foobar( arg1 , arg2 )
//         ---- - ----
//         arguments
//
// This should not lead to noticable differences since we make no effort to
// do the insane special handling of 'url()' functions that's in the
// css spec
#[derive(Debug, Clone)]
pub struct FunctionToken {
    pub(super) name: CssIdent,
    pub(super) paren: token::Paren,
    pub(super) args: Vec<ComponentValue>,
}

impl ToTokens for FunctionToken {
    fn to_tokens(&self, toks: &mut TokenStream) {
        self.name.to_tokens(toks);
        self.paren.surround(toks, |toks| {
            for c in self.args.iter() {
                c.to_tokens(toks);
            }
        });
    }
}

impl Parse for FunctionToken {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        Self::parse_with_name(input.parse()?, input)
    }
}

impl FunctionToken {
    pub(super) fn parse_with_name(name: CssIdent, input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let paren = parenthesized!(inner in input);
        let args = ComponentValue::parse_multiple(&inner)?;
        Ok(Self { name, paren, args })
    }
}
