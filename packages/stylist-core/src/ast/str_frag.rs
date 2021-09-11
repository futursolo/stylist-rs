use std::borrow::Cow;

use super::{StyleContext, ToStyleStr};

/// A String Fragment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringFragment {
    pub inner: Cow<'static, str>,
}

impl ToStyleStr for StringFragment {
    fn write_style(&self, w: &mut String, _ctx: &mut StyleContext<'_, '_>) {
        w.push_str(&self.inner);
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for StringFragment {
    fn from(s: T) -> Self {
        Self { inner: s.into() }
    }
}
