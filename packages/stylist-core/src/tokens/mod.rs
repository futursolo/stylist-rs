mod comment;
mod error;
mod ident;
mod input;
mod location;
mod punct;
mod spacing;
mod token;
mod token_stream;
mod tokenize;

pub use error::{ITokenizeResult, TokenizeError, TokenizeResult};

pub use comment::Comment;
pub use ident::Ident;
pub use input::{InputStr, InputTokens};
pub use location::Location;
pub use punct::Punct;
pub use spacing::Spacing;
pub use token::{Token, TokenTree};
pub use token_stream::TokenStream;
pub use tokenize::Tokenize;

mod rtokens {
    pub use proc_macro2::{
        Literal as RLiteral, Span as RSpan, TokenStream as RTokenStream, TokenTree as RTokenTree,
    };
}
