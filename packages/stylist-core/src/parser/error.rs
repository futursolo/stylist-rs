use thiserror::Error;

use crate::tokens::Location;

#[derive(Debug, Error)]
#[error("{}", .msg)]
pub struct ParseError {
    msg: String,
    location: Location,
}

impl ParseError {
    pub fn new<S>(msg: S, location: Location) -> Self
    where
        S: Into<String>,
    {
        Self {
            msg: msg.into(),
            location,
        }
    }

    pub fn unexpected_token(location: Location) -> Self {
        Self::new("unexpected token", location)
    }
}
