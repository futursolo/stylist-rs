use std::fmt;

use super::ToStyleStr;

/// A CSS Selector.
///
/// E.g.:
/// ```css
/// div[attr="val"].my-class#some-id
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Selector {
    pub inner: String,
}

impl ToStyleStr for Selector {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        if self.inner.contains('&') {
            let scoped_class = format!(".{}", class_name);
            write!(w, "{}", self.inner.replace("&", scoped_class.as_str()))?;
        } else {
            write!(w, ".{} {}", class_name, self.inner)?;
        }

        Ok(())
    }
}

impl<T: Into<String>> From<T> for Selector {
    fn from(s: T) -> Self {
        Self { inner: s.into() }
    }
}
