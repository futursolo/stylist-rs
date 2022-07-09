use crate::tokens::{Iter, TokenStream, TokenTree};
use std::iter::Peekable;

type ParseIter<'a, T> = Peekable<Iter<'a, T>>;

#[derive(Debug, Clone)]
pub struct ParseStream<'a> {
    cursor: Peekable<Iter<'a, TokenTree>>,
}

impl<'a> ParseStream<'a> {
    /// Returns an `Iterator` over tokens.
    pub fn iter(&self) -> ParseIter<'a, TokenTree> {
        self.cursor.clone()
    }

    /// Trim until next token is not space or comment.
    pub fn trim_start(mut self) -> Self {
        while self.cursor.next_if(|m| m.is_trimmable()).is_some() {}

        self
    }

    /// Returns a reference of the next token without removing it from the input.
    pub fn first(&mut self) -> Option<&TokenTree> {
        self.cursor.peek().cloned()
    }

    /// Pops the next token.
    pub fn pop_front(mut self) -> (Option<TokenTree>, Self) {
        let token = self.cursor.next().cloned();

        (token, self)
    }

    /// Pops the next token if op returns `Some(T)`.
    ///
    /// Returns the value in form of `T`
    pub fn pop_by<O, T>(mut self, op: O) -> (Option<T>, Self)
    where
        O: FnOnce(&TokenTree) -> Option<T>,
    {
        match self.first().and_then(op) {
            Some(m) => {
                self.cursor.next();
                (Some(m), self)
            }
            None => (None, self),
        }
    }

    /// Returns `true` if all tokens have been parsed.
    pub fn is_empty(&mut self) -> bool {
        self.cursor.peek().is_none()
    }
}

impl<'a> From<&'a TokenStream> for ParseStream<'a> {
    fn from(m: &'a TokenStream) -> Self {
        Self {
            cursor: m.iter().peekable(),
        }
    }
}
