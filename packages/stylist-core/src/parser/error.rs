use thiserror::Error;

use crate::tokens::Location;

/// An error returned when the parser / tokeniser encountered an error.
#[derive(Debug, Error)]
#[error("{}", .msg)]
pub struct ParseError {
    msg: String,
    location: Location,
}

impl ParseError {
    pub fn new<S: Into<String>>(msg: S, location: Location) -> Self {
        Self {
            msg: msg.into(),
            location,
        }
    }

    pub fn unexpected_token(location: Location) -> Self {
        Self::new("unexpected token", location)
    }
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;
