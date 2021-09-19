use std::ops::Deref;

use arcstr::Substr;
#[cfg(feature = "proc_macro_support")]
use proc_macro2 as r;

use super::Location;

pub trait Input {
    /// Returns `true` if the input is empty.
    fn is_empty(&self) -> bool;

    /// Returns the `Location` of the first token in the input.
    ///
    /// Returns `None` if the input is empty.
    fn first_token_location(&self) -> Option<Location>;
}

/// The input to be passed to [`tokenize`](super::Tokenize::tokenize) created from a string literal.
#[derive(Debug, Clone)]
pub struct InputStr {
    inner: Substr,
    #[cfg(feature = "proc_macro_support")]
    token: Option<r::TokenStream>,
}

impl From<String> for InputStr {
    fn from(m: String) -> Self {
        Self {
            inner: m.into(),
            #[cfg(feature = "proc_macro_support")]
            token: None,
        }
    }
}

impl Input for InputStr {
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn first_token_location(&self) -> Option<Location> {
        if self.is_empty() {
            None
        } else {
            Some(Location::Literal {
                #[cfg(feature = "proc_macro_support")]
                token: self.token.clone(),
                range: self.inner.substr(0..1).range(),
            })
        }
    }
}

impl Deref for InputStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl InputStr {
    /// Split the input at the given location.
    ///
    /// Returns the split off string, the location of the string, and the rest in the form of
    /// [`InputStr`].
    pub fn split_at(self, mid: usize) -> (Substr, Location, InputStr) {
        let left = self.inner.substr(0..mid);
        let right = self.inner.substr(mid..);

        let location = Location::Literal {
            #[cfg(feature = "proc_macro_support")]
            token: self.token.clone(),
            range: left.range(),
        };

        (
            left,
            location,
            Self {
                inner: right,
                #[cfg(feature = "proc_macro_support")]
                token: self.token,
            },
        )
    }

    /// Returns the underlying [`TokenStream`](proc_macro2::TokenStream) of the string literal,
    /// unavailable if the input is created from a runtime string.
    #[cfg(feature = "proc_macro_support")]
    pub fn token(&self) -> Option<r::TokenStream> {
        self.token.clone()
    }
}

#[cfg(feature = "proc_macro_support")]
mod feat_proc_macro {
    use super::*;
    use litrs::StringLit;
    use std::convert::TryFrom;

    impl TryFrom<r::TokenTree> for InputStr {
        type Error = r::TokenStream;

        fn try_from(value: r::TokenTree) -> Result<Self, Self::Error> {
            let s = match StringLit::try_from(value.clone()) {
                Ok(m) => m,
                Err(e) => return Err(e.to_compile_error2()),
            };

            Ok(Self {
                inner: s.to_string().into(),
                token: Some(value.into()),
            })
        }
    }
}
