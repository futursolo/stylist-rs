use std::borrow::Cow;
use std::fmt;

use super::ToStyleStr;
use crate::{Error, Result};

/// The kind of the string
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringKind {
    /// This is a string literal
    ///
    /// example: `"some string"`
    Literal,
    /// This is a interpolation syntax
    ///
    /// example: `${some_var}`
    Interpolation,
}

/// A String Fragment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringFragment {
    pub inner: Cow<'static, str>,
    pub kind: StringKind,
}

impl ToStyleStr for StringFragment {
    fn write_style<W: fmt::Write>(&self, w: &mut W, _class_name: Option<&str>) -> Result<()> {
        if self.kind == StringKind::Interpolation {
            return Err(Error::Interpolation {
                name: self.inner.to_string(),
            });
        }

        write!(w, "{}", self.inner)?;

        Ok(())
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for StringFragment {
    fn from(s: T) -> Self {
        Self {
            inner: s.into(),
            kind: StringKind::Literal,
        }
    }
}
