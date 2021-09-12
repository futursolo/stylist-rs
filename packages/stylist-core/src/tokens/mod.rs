mod ident;
mod input;
mod location;
mod token;
mod tokenize;

pub use ident::Ident;
pub use input::{InputStr, InputTokens};
pub use location::Location;
pub use token::{Token, TokenTree};
pub use tokenize::Tokenize;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream {
    inner: Vec<TokenTree>,
}

mod rtokens {
    pub use proc_macro2::{TokenStream as RTokenStream, TokenTree as RTokenTree};
}
