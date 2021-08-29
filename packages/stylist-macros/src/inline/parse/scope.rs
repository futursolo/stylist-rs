use super::CssScopeContent;
use syn::{
    braced,
    parse::{Parse, ParseBuffer, Result as ParseResult},
    token,
};

#[derive(Debug)]
pub struct CssScope {
    brace: token::Brace,
    pub contents: Vec<CssScopeContent>,
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let brace = braced!(inner in input);
        let contents = CssScopeContent::consume_list_of_rules(&inner)?;
        Ok(Self { brace, contents })
    }
}
