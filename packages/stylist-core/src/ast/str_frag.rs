use std::borrow::Cow;
use std::fmt;

use super::{StyleContext, ToStyleStr};
use crate::Result;

/// A String Fragment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringFragment {
    pub inner: Cow<'static, str>,
}

impl ToStyleStr for StringFragment {
    fn write_style<W: fmt::Write>(&self, w: &mut W, _ctx: &StyleContext<'_>) -> Result<()> {
        write!(w, "{}", self.inner)?;

        Ok(())
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for StringFragment {
    fn from(s: T) -> Self {
        Self { inner: s.into() }
    }
}
