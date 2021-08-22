use std::borrow::Cow;
use std::fmt;

use super::ToStyleStr;
use crate::Result;

/// A simple CSS property in the form of a key value pair. Mirrors what would
/// be called a "Declaration" in the CSS standard.
///
/// E.g.: `color: red`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleAttribute {
    pub key: Cow<'static, str>,
    pub value: Cow<'static, str>,
}

impl ToStyleStr for StyleAttribute {
    fn write_style<W: fmt::Write>(&self, w: &mut W, _class_name: Option<&str>) -> Result<()> {
        write!(w, "{}: {};", self.key, self.value)?;

        Ok(())
    }
}
