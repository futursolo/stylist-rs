use super::TokenStream;

pub trait Tokenize<T> {
    fn tokenize(input: T) -> Result<(TokenStream, T), T>;
}
