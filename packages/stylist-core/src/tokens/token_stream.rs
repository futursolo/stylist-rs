use std::iter::{FromIterator, IntoIterator};

use super::TokenTree;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TokenStream {
    inner: Vec<TokenTree>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().flatten().collect(),
        }
    }
}

impl From<TokenTree> for TokenStream {
    fn from(m: TokenTree) -> Self {
        Self { inner: vec![m] }
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

// impl TokenStream {
//     fn push(&mut self, item: TokenTree) {
//         self.inner.push(item);
//     }
// }
