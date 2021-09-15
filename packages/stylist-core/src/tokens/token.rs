use super::{
    Comment, ITokenizeResult, Ident, InputStr, Location, Punct, Spacing, TokenStream, Tokenize,
    TokenizeResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    Ident(Ident),
    Spacing(Spacing),
    Punct(Punct),
    Comment(Comment),
}

pub trait Token {
    fn location(&self) -> &Location;
    fn as_str(&self) -> &str;
}

impl Tokenize<InputStr> for TokenTree {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        Ident::tokenize(input)
            .terminal_or_else(Spacing::tokenize)
            .terminal_or_else(Punct::tokenize)
            .terminal_or_else(Comment::tokenize)
    }
}
