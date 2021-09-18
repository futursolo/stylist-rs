use super::TokenStream;

use super::{TokenizeError, TokenizeResult};

/// Tokenise a value from input.
pub trait Tokenize<T> {
    /// Tokenise a value from input(`T`).
    ///
    /// Returns [`Ok((TokenStream, T))`] if successful with `T` being
    /// the remaining part of the input.
    ///
    /// Returns [`Err(TokenizeError)`] if failed to tokenise
    fn tokenize(input: T) -> TokenizeResult<T, TokenStream>;

    /// Call [`tokenize`](Tokenize::tokenize) until an error is returned.
    ///
    /// Returns `Ok()` if at least 1 token is successful and the final error is not terminal.
    fn tokenize_until_error(input: T) -> TokenizeResult<T, TokenStream> {
        let mut tokens = TokenStream::new();
        let mut rest = input;

        loop {
            let (token, rest_next) = match Self::tokenize(rest) {
                Ok(m) => m,
                Err(TokenizeError::NotTokenized(e)) => {
                    if tokens.is_empty() {
                        return Err(TokenizeError::NotTokenized(e));
                    } else {
                        return Ok((tokens, e));
                    }
                }
                Err(TokenizeError::Terminal(e)) => return Err(TokenizeError::Terminal(e)),
            };

            tokens.extend(token);
            rest = rest_next;
        }
    }
}
