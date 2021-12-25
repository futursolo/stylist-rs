use crate::arc_ref::ArcRef;
use crate::tokens::{TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub struct ParseStream<'a> {
    inner: ArcRef<'a, TokenStream>,
    cursor: usize,
}

impl ParseStream<'_> {
    fn advance(&mut self, len: usize) {
        self.cursor += len;
    }

    /// Returns an `Iterator` over tokens.
    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.inner.iter().skip(self.cursor)
    }

    /// Trim until next token is not space or comment.
    pub fn trim_start(mut self) -> Self {
        self.advance(self.iter().take_while(|m| m.is_trimmable()).count());

        self
    }

    /// Returns a reference of the next token without removing it from the input.
    pub fn first(&self) -> Option<&TokenTree> {
        self.inner.iter().next()
    }

    /// Pops the next token.
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

    /// Returns `true` if all tokens have been parsed.
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}

impl<'a> From<ArcRef<'a, TokenStream>> for ParseStream<'a> {
    fn from(m: ArcRef<'a, TokenStream>) -> Self {
        Self {
            inner: m,
            cursor: 0,
        }
    }
}
