use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use arcstr::Substr;
use litrs::StringLit;

use super::rtokens::{RTokenStream, RTokenTree};
use super::Location;

#[derive(Debug, Clone)]
pub struct InputStr {
    inner: Substr,
    token: Option<RTokenStream>,
}

impl From<String> for InputStr {
    fn from(m: String) -> Self {
        Self {
            inner: m.into(),
            token: None,
        }
    }
}

impl TryFrom<RTokenTree> for InputStr {
    type Error = RTokenStream;

    fn try_from(value: RTokenTree) -> Result<Self, Self::Error> {
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

impl Deref for InputStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl InputStr {
    pub fn split_at(self, mid: usize) -> (Substr, Location, InputStr) {
        let left = self.inner.substr(0..mid);
        let location = Location::Literal {
            token: self.token.clone(),
            range: left.range(),
        };

        (
            left,
            location,
            Self {
                inner: self.inner.substr(mid..),
                token: self.token.clone(),
            },
        )
    }

    pub fn token(&self) -> Option<RTokenStream> {
        self.token.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InputTokens {
    inner: VecDeque<RTokenTree>,
}

impl Deref for InputTokens {
    type Target = VecDeque<RTokenTree>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for InputTokens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
