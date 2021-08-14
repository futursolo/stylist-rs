use std::fmt;

use super::ToStyleStr;

/// A simple CSS property in the form of a key value pair. Mirrors what would
/// be called a "Declaration" in the CSS standard.
///
/// E.g.: `color: red`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleAttribute {
    pub key: String,
    pub value: String,
}

impl ToStyleStr for StyleAttribute {
    fn write_style<W: fmt::Write>(&self, w: &mut W, _class_name: &str) -> fmt::Result {
        write!(w, "{}: {};", self.key, self.value)
    }
}
