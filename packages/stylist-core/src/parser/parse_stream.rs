use std::sync::Arc;

use crate::tokens::{TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub struct ParseStream {
    inner: Arc<Vec<TokenTree>>,
    cursor: usize,
}

impl ParseStream {
    fn advance(&mut self, len: usize) {
        self.cursor += len;
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.inner.iter().skip(self.cursor)
    }

    pub fn trim_start(&self) -> Self {
        let mut self_ = self.clone();

        self_.advance(self.iter().take_while(|m| m.is_trimmable()).count());

        self_
    }
}

impl From<TokenStream> for ParseStream {
    fn from(m: TokenStream) -> Self {
        Self {
            inner: m.into_iter().collect::<Vec<TokenTree>>().into(),
            cursor: 0,
        }
    }
}
