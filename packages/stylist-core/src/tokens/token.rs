use super::{
    Comment, Group, ITokenizeResult, Ident, InputStr, InputTokens, Location, Punct, Space,
    TokenStream, Tokenize, TokenizeResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    Ident(Ident),
    Space(Space),
    Punct(Punct),
    Comment(Comment),
    Group(Group),
}

/// A trait that represents a token.
pub trait Token {
    /// Returns the [`Location`] of current token.
    fn location(&self) -> &Location;

    /// Returns the token content in the form of a string.
    fn as_str(&self) -> &str;
}

impl Token for TokenTree {
    fn location(&self) -> &Location {
        match self {
            Self::Ident(m) => m.location(),
            Self::Space(m) => m.location(),
            Self::Punct(m) => m.location(),
            Self::Comment(m) => m.location(),
            Self::Group(m) => m.location(),
        }
    }
    fn as_str(&self) -> &str {
        match self {
            Self::Ident(m) => m.as_str(),
            Self::Space(m) => m.as_str(),
            Self::Punct(m) => m.as_str(),
            Self::Comment(m) => m.as_str(),
            Self::Group(m) => m.as_str(),
        }
    }
}

impl Tokenize<InputStr> for TokenTree {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        Ident::tokenize(input)
            .terminal_or_else(Space::tokenize)
            .terminal_or_else(Comment::tokenize)
            .terminal_or_else(Punct::tokenize)
            .terminal_or_else(Ident::tokenize)
            .terminal_or_else(Group::tokenize)
    }
}

impl Tokenize<InputTokens> for TokenTree {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        Ident::tokenize(input)
            // .terminal_or_else(Spacing::tokenize)
            // .terminal_or_else(Comment::tokenize)
            .terminal_or_else(Punct::tokenize)
            .terminal_or_else(Ident::tokenize)
            .terminal_or_else(Group::tokenize)
    }
}
