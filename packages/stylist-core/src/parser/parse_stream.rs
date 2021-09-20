use std::sync::Arc;

use crate::tokens::{TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub struct ParseStream {
    inner: Arc<TokenStream>,
    cursor: usize,
}

impl ParseStream {
    fn advance(&mut self, len: usize) {
        self.cursor += len;
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.inner.iter().skip(self.cursor)
    }

    pub fn trim_start(mut self) -> Self {
        self.advance(self.iter().take_while(|m| m.is_trimmable()).count());

        self
    }

    /// Get a reference of the next token without removing it from the input.
    pub fn first(&self) -> Option<&TokenTree> {
        self.inner.iter().next()
    }

    pub fn pop_front(mut self) -> (Option<TokenTree>, Self) {
        let token = self.first().cloned();

        if token.is_some() {
            self.advance(1);
        }

        (token, self)
    }

    /// Pops the next token if op returns `Some(T)`.
    ///
    /// Returns the value in form of `T`
    pub fn pop_by<O, T>(mut self, op: O) -> (Option<T>, Self)
    where
        O: Fn(&TokenTree) -> Option<T>,
    {
        match self.first().and_then(op) {
            Some(m) => {
                self.advance(1);
                (Some(m), self)
            }
            None => (None, self),
        }
    }
}

impl From<TokenStream> for ParseStream {
    fn from(m: TokenStream) -> Self {
        Self {
            inner: m.into(),
            cursor: 0,
        }
    }
}
