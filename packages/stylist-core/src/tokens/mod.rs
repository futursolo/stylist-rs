mod comment;
mod error;
mod group;
mod ident;
mod input;
mod interpolation;
mod literal;
mod location;
mod punct;
mod space;
mod token;
mod token_stream;
mod tokenize;

pub use error::{ITokenizeResult, TokenizeError, TokenizeResult};

pub use comment::Comment;
pub use group::{Delimiter, Group};
pub use ident::Ident;
pub use input::{InputStr, InputTokens};
pub use interpolation::Interpolation;
pub use literal::Literal;
pub use location::Location;
pub use punct::Punct;
pub use space::Space;
pub use token::{Token, TokenTree};
pub use token_stream::TokenStream;
pub use tokenize::Tokenize;

mod rtokens {
    pub use proc_macro2::{
        Delimiter as RDelimiter, Literal as RLiteral, Span as RSpan, TokenStream as RTokenStream,
        TokenTree as RTokenTree,
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_partial_eq {
    ($typ: ident, $($ident: ident),+) => {
        impl PartialEq for $typ {
            fn eq(&self, other: &Self) -> bool {
                $(self.$ident == other.$ident) &&+
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_token {
    ($typ: ident) => {
        impl crate::tokens::Token for $typ {
            fn as_str(&self) -> &str {
                &self.inner
            }
            fn location(&self) -> &crate::tokens::Location {
                &self.location
            }
        }
    };
}
