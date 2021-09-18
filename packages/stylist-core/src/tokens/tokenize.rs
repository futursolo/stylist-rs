use super::TokenStream;

use super::TokenizeResult;

/// Tokenise a value from input
pub trait Tokenize<T> {
    /// Tokenise a value from input(`T`)
    ///
    /// Returns [`Ok(TokenStream, T)`] if successful with `T` being
    /// the remaining part of the input.
    ///
    /// Returns [`Err(T)`] if failed to tokenise
    fn tokenize(input: T) -> TokenizeResult<T, TokenStream>;
}
