use super::TokenTree;

pub trait Tokenize<T> {
    fn tokenize(input: T) -> Result<(TokenTree, T), T>;
}
