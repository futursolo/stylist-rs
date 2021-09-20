use std::collections::VecDeque;

use proc_macro2 as r;

use stylist_core::tokens::{Input, Location};

/// The input to be passed to [`tokenize`](super::Tokenize::tokenize) created from a [`proc_macro2::TokenStream`].
#[derive(Debug, Clone)]
pub struct InputTokens {
    inner: VecDeque<r::TokenTree>,
}

impl Input for InputTokens {
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn first_token_location(&self) -> Option<Location> {
        self.first()
            .cloned()
            .map(|m| Location::TokenStream(m.into()))
    }
}

impl InputTokens {
    /// Pops a token from the front of the input.
    pub fn pop_front(mut self) -> (Option<r::TokenTree>, InputTokens) {
        let token = self.inner.pop_front();

        (token, self)
    }

    /// Get a reference of the next token without removing it from the input.
    pub fn first(&self) -> Option<&r::TokenTree> {
        self.inner.get(0)
    }

    /// Pops the next token if op returns `Some(T)`.
    ///
    /// Returns the value in form of `T`
    pub fn pop_by<O, T>(self, op: O) -> (Option<T>, InputTokens)
    where
        O: Fn(&r::TokenTree) -> Option<T>,
    {
        match self.first().and_then(op) {
            Some(m) => {
                let (_, tokens) = self.pop_front();
                (Some(m), tokens)
            }
            None => (None, self),
        }
    }
}

impl From<r::TokenStream> for InputTokens {
    fn from(m: r::TokenStream) -> Self {
        Self {
            inner: m.into_iter().collect(),
        }
    }
}
