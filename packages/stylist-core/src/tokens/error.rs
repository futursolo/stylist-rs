use thiserror::Error;

use super::Input;
use crate::parser::{ParseError, ParseResult};

#[derive(Debug, Error)]
pub enum TokenizeError<I> {
    /// Failed to Tokenize to into a token
    NotTokenized(I),
    /// Failed to Tokenize to into a token and the error is not recoverable.
    Terminal(ParseError),
}

impl<I> From<ParseError> for TokenizeError<I> {
    fn from(m: ParseError) -> Self {
        Self::Terminal(m)
    }
}

pub type TokenizeResult<I, T> = std::result::Result<(T, I), TokenizeError<I>>;

pub trait ITokenizeResult<I, T> {
    /// Tries tokenize the input with `op` if result is `Err` and [`TokenizeError`] is not `Terminal`.
    fn terminal_or_else<O>(self, op: O) -> TokenizeResult<I, T>
    where
        O: FnOnce(I) -> TokenizeResult<I, T>;

    /// Returns `Ok()` unless the error is terminal.
    fn terminal_or_ok(self) -> ParseResult<(T, I)>;

    /// Returns a terminal error unless the remaining input is empty.
    ///
    /// Returns `Ok()` if the remaining input is empty.
    fn empty_or_terminal(self) -> ParseResult<T>;
}

impl<I: Input, T: Default> ITokenizeResult<I, T> for TokenizeResult<I, T> {
    fn terminal_or_else<O>(self, op: O) -> TokenizeResult<I, T>
    where
        O: FnOnce(I) -> TokenizeResult<I, T>,
    {
        match self {
            Err(TokenizeError::NotTokenized(m)) => op(m),
            _ => self,
        }
    }

    fn terminal_or_ok(self) -> ParseResult<(T, I)> {
        match self {
            Ok(m) => Ok(m),
            Err(TokenizeError::NotTokenized(m)) => Ok((T::default(), m)),
            Err(TokenizeError::Terminal(e)) => Err(e),
        }
    }

    fn empty_or_terminal(self) -> ParseResult<T> {
        match self {
            Ok(m) => Ok(m.0),
            Err(TokenizeError::NotTokenized(m)) => m
                .first_token_location()
                .map(|m| Err(ParseError::unexpected_token(m)))
                .unwrap_or_else(|| Ok(T::default())),
            Err(TokenizeError::Terminal(e)) => Err(e),
        }
    }
}
