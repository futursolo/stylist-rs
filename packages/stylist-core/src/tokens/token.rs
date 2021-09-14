use super::{Comment, Ident, InputStr, Location, Punct, Spacing, TokenStream, Tokenize};

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
    fn tokenize(input: InputStr) -> Result<(TokenStream, InputStr), InputStr> {
        Ident::tokenize(input)
            .or_else(Spacing::tokenize)
            .or_else(Punct::tokenize)
            .or_else(Comment::tokenize)
    }
}
