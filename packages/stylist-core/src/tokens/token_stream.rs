use super::TokenTree;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream {
    inner: Vec<TokenTree>,
}

impl From<TokenTree> for TokenStream {
    fn from(m: TokenTree) -> Self {
        Self { inner: vec![m] }
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        for elem in iter {
            self.inner.push(elem);
        }
    }
}

// impl TokenStream {
//     fn push(&mut self, item: TokenTree) {
//         self.inner.push(item);
//     }
// }
