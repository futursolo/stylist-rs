use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::Deref;

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
        let right = self.inner.substr(mid..);

        let location = Location::Literal {
            token: self.token.clone(),
            range: left.range(),
        };

        (
            left,
            location,
            Self {
                inner: right,
                token: self.token,
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

impl InputTokens {
    pub fn pop_front(mut self) -> (Option<RTokenTree>, InputTokens) {
        let token = self.inner.pop_front();

        (token, self)
    }

    pub fn peek(&self) -> Option<&RTokenTree> {
        self.get(0)
    }

    // Pop if op returns Some(T).
    pub fn pop_by<O, T>(self, op: O) -> (Option<T>, InputTokens)
    where
        O: Fn(RTokenTree) -> Option<T>,
    {
        match self.get(0).cloned().and_then(op) {
            Some(m) => {
                let (_, tokens) = self.pop_front();
                (Some(m), tokens)
            }
            None => (None, self),
        }
    }
}

impl Deref for InputTokens {
    type Target = VecDeque<RTokenTree>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<RTokenStream> for InputTokens {
    fn from(m: RTokenStream) -> Self {
        Self {
            inner: m.into_iter().collect(),
        }
    }
}
