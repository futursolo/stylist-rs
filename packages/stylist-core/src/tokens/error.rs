use thiserror::Error;

use crate::parser::ParseError;

#[derive(Debug, Error)]
pub enum TokenizeError<I> {
    /// Failed to Tokenize to into a token
    NotTokenized(I),
    /// Failed to Tokenize to into a token and the error is not recoverable.
    Terminal(ParseError),
}

pub type TokenizeResult<I, T> = std::result::Result<(T, I), TokenizeError<I>>;

pub trait ITokenizeResult<I, T> {
    /// Tries tokenize the input with `op` if result is `Err` and [`TokenizeError`] is not `Terminal`.
    fn terminal_or_else<O>(self, op: O) -> TokenizeResult<I, T>
    where
        O: FnOnce(I) -> TokenizeResult<I, T>;
}

impl<I, T> ITokenizeResult<I, T> for TokenizeResult<I, T> {
    fn terminal_or_else<O>(self, op: O) -> TokenizeResult<I, T>
    where
        O: FnOnce(I) -> TokenizeResult<I, T>,
    {
        match self {
            Err(TokenizeError::NotTokenized(m)) => op(m),
            _ => self,
        }
    }
}
