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
    #[cfg(feature = "proc_macro_support")]
    args: std::sync::Arc<Arguments>,
}

impl From<String> for InputStr {
    fn from(m: String) -> Self {
        Self {
            inner: m.into(),
            #[cfg(feature = "proc_macro_support")]
            token: None,
            #[cfg(feature = "proc_macro_support")]
            args: Default::default(),
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
                #[cfg(feature = "proc_macro_support")]
                args: self.args,
            },
        )
    }
}

#[cfg(feature = "proc_macro_support")]
mod feat_proc_macro {
    use super::*;
    use litrs::StringLit;
    use std::collections::{HashMap, HashSet};
    use std::convert::TryFrom;
    use std::sync::{Arc, Mutex};

    impl InputStr {
        /// Returns the underlying [`TokenStream`](proc_macro2::TokenStream) of the string literal,
        /// unavailable if the input is created from a runtime string.
        #[cfg(feature = "proc_macro_support")]
        pub fn token(&self) -> Option<r::TokenStream> {
            self.token.clone()
        }

        pub fn args(&self) -> Arc<Arguments> {
            self.args.clone()
        }

        pub fn with_args(self, args: Arguments) -> Self {
            Self {
                inner: self.inner,
                token: self.token,
                args: args.into(),
            }
        }
    }

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
                args: Default::default(),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct Argument {
        pub name: String,
        pub name_token: r::Ident,
        pub tokens: r::TokenStream,
    }

    #[derive(Debug, Default)]
    pub struct Arguments {
        args: HashMap<String, Argument>,
        args_used: Mutex<HashSet<String>>,
    }

    impl From<HashMap<String, Argument>> for Arguments {
        fn from(value: HashMap<String, Argument>) -> Self {
            Self {
                args: value,
                ..Default::default()
            }
        }
    }

    impl Arguments {
        pub fn get<S: AsRef<str>>(&self, name: S) -> Option<Argument> {
            let name = name.as_ref();

            self.args.get(name).cloned().map(|m| {
                let mut used = self.args_used.lock().unwrap();

                used.insert(name.to_owned());

                m
            })
        }

        pub fn get_unused_args(&self) -> Vec<Argument> {
            let used = self.args_used.lock().unwrap();

            self.args
                .iter()
                .filter_map(|(key, value)| (!used.contains(key)).then(|| value))
                .cloned()
                .collect()
        }
    }
}

#[cfg(feature = "proc_macro_support")]
pub use feat_proc_macro::{Argument, Arguments};
