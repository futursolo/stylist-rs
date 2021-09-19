use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::Deref;

use arcstr::Substr;
use litrs::StringLit;

use super::rtokens::{RTokenStream, RTokenTree};
use super::Location;

pub trait Input {
    /// Returns `true` if the input is empty.
    fn is_empty(&self) -> bool;

    /// Returns the `Location` of the first token in the input.
    ///
    /// Returns `None` if the input is empty.
    fn first_token_location(&self) -> Option<Location>;
}

/// The input to be passed to [`tokenize`](super::Tokenize::tokenize) created from a string literal.
#[derive(Debug, Clone)]
pub struct InputStr {
    inner: Substr,
    token: Option<RTokenStream>,
}

impl From<String> for InputStr {
    fn from(m: String) -> Self {
        Self {
            inner: m.into(),
            token: None,
        }
    }
}

impl Input for InputStr {
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn first_token_location(&self) -> Option<Location> {
        if self.is_empty() {
            None
        } else {
            Some(Location::Literal {
                token: self.token.clone(),
                range: self.inner.substr(0..1).range(),
            })
        }
    }
}

impl TryFrom<RTokenTree> for InputStr {
    type Error = RTokenStream;

    fn try_from(value: RTokenTree) -> Result<Self, Self::Error> {
        let s = match StringLit::try_from(value.clone()) {
            Ok(m) => m,
            Err(e) => return Err(e.to_compile_error2()),
        };

        Ok(Self {
            inner: s.to_string().into(),
            token: Some(value.into()),
        })
    }
}

impl Deref for InputStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl InputStr {
    /// Split the input at the given location.
    ///
    /// Returns the split off string, the location of the string, and the rest in the form of
    /// [`InputStr`].
    pub fn split_at(self, mid: usize) -> (Substr, Location, InputStr) {
        let left = self.inner.substr(0..mid);
        let right = self.inner.substr(mid..);

        let location = Location::Literal {
            token: self.token.clone(),
            range: left.range(),
        };

        (
            left,
            location,
            Self {
                inner: right,
                token: self.token,
            },
        )
    }

    /// Returns the underlying [`TokenStream`](proc_macro2::TokenStream) of the string literal,
    /// unavailable if the input is created from a runtime string.
    pub fn token(&self) -> Option<RTokenStream> {
        self.token.clone()
    }
}

/// The input to be passed to [`tokenize`](super::Tokenize::tokenize) created from a [`proc_macro2::TokenStream`].
#[derive(Debug, Clone)]
pub struct InputTokens {
    inner: VecDeque<RTokenTree>,
}

impl Input for InputTokens {
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn first_token_location(&self) -> Option<Location> {
        self.peek()
            .cloned()
            .map(|m| Location::TokenStream(m.into()))
    }
}

impl InputTokens {
    /// Pops a token from the front of the input.
    pub fn pop_front(mut self) -> (Option<RTokenTree>, InputTokens) {
        let token = self.inner.pop_front();

        (token, self)
    }

    /// Peeks the next token without removing it from the input.
    pub fn peek(&self) -> Option<&RTokenTree> {
        self.inner.get(0)
    }

    /// Pops the next token if op returns `Some(T)`.
    ///
    /// Returns the value in form of `T`
    pub fn pop_by<O, T>(self, op: O) -> (Option<T>, InputTokens)
    where
        O: Fn(RTokenTree) -> Option<T>,
    {
        match self.peek().cloned().and_then(op) {
            Some(m) => {
                let (_, tokens) = self.pop_front();
                (Some(m), tokens)
            }
            None => (None, self),
        }
    }
}

// impl Deref for InputTokens {
//     type Target = VecDeque<RTokenTree>;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

impl From<RTokenStream> for InputTokens {
    fn from(m: RTokenStream) -> Self {
        Self {
            inner: m.into_iter().collect(),
        }
    }
}
