/// A module to facilitate tokenisation of CSS stylesheets.
// This module mostly mirrors the design / name / API of rust's built-in `proc_macro`.
mod comment;
mod error;
mod group;
mod ident;
mod input;
mod literal;
mod location;
mod punct;
mod space;
mod token;
mod token_stream;
mod tokenize;

#[cfg(feature = "proc_macro_support")]
mod frag;
#[cfg(feature = "proc_macro_support")]
mod interpolation;

pub use error::{ITokenizeResult, TokenizeError, TokenizeResult};

pub use comment::Comment;
pub use group::{Delimiter, Group};
pub use ident::Ident;
pub use input::{Input, InputStr};
pub use literal::Literal;
pub use location::Location;
pub use punct::Punct;
pub use space::Space;
pub use token::{Token, TokenTree};
pub use token_stream::{Iter, TokenStream};
pub use tokenize::Tokenize;

#[cfg(feature = "proc_macro_support")]
pub use frag::Fragment;
#[cfg(feature = "proc_macro_support")]
pub use input::{Argument, Arguments};
#[cfg(feature = "proc_macro_support")]
pub use interpolation::Interpolation;

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

#[cfg(not(feature = "proc_macro_support"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_token {
    ($typ: ident) => {
        impl $crate::tokens::Token for $typ {
            fn to_fragments(&self) -> Vec<&str> {
                vec![&self.inner]
            }
            fn location(&self) -> &$crate::tokens::Location {
                &self.location
            }
        }
    };
}

#[cfg(feature = "proc_macro_support")]
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_token {
    ($typ: ident) => {
        impl $crate::tokens::Token for $typ {
            fn to_fragments(&self) -> Vec<$crate::tokens::Fragment> {
                vec![$crate::tokens::Fragment::Literal(self.inner.to_string())]
            }
            fn location(&self) -> &$crate::tokens::Location {
                &self.location
            }
        }
    };
}
