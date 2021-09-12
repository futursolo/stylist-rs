use super::{Ident, Location};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    Ident(Ident),
}

pub trait Token {
    fn location(&self) -> &Location;
    fn as_str(&self) -> &str;
}
