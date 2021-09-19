use arcstr::Substr;
#[cfg(feature = "proc_macro_support")]
use typed_builder::TypedBuilder;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::{__impl_partial_eq, __impl_token};

/// A token that represents a punctuation mark.
///
/// It is a single punctuation character like `+`, `-` or `#`.
#[cfg_attr(feature = "proc_macro_support", derive(TypedBuilder))]
#[derive(Debug, Clone)]
pub struct Punct {
    inner: Substr,
    location: Location,
}

impl Punct {
    /// Returns the value of this punctuation as [`char`].
    pub fn as_char(&self) -> char {
        self.inner.chars().next().unwrap()
    }
}

__impl_partial_eq!(Punct, inner);
__impl_token!(Punct);

impl Tokenize<InputStr> for Punct {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let valid_char = |c: &char| "&#+,-.:;<@\\".contains(*c);

        if input.chars().next().filter(valid_char).is_some() {
            let (inner, location, rest) = input.split_at(1);

            Ok((TokenTree::Punct(Punct { inner, location }).into(), rest))
        } else {
            Err(TokenizeError::NotTokenized(input))
        }
    }
}
