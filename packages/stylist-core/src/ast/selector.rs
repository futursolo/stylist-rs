use std::borrow::Cow;
use std::fmt;

use super::{StringKind, ToStyleStr};
use crate::{Error, Result};

/// A CSS Selector.
///
/// E.g.:
/// ```css
/// div[attr="val"].my-class#some-id
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Selector {
    pub inner: Cow<'static, str>,
    pub kind: StringKind,
}

impl ToStyleStr for Selector {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> Result<()> {
        if self.kind == StringKind::Interpolation {
            return Err(Error::Interpolation {
                name: self.inner.to_string(),
            });
        }

        if let Some(m) = class_name {
            // If contains current selector or root pseudo class, replace them with class name.
            if self.inner.contains('&') || self.inner.contains(":root") {
                let scoped_class = format!(".{}", m);

                write!(
                    w,
                    "{}",
                    self.inner
                        .replace("&", scoped_class.as_str())
                        .replace(":root", scoped_class.as_str())
                )?;

            // If selector starts with a pseudo-class, apply it to the root element.
            } else if self.inner.starts_with(':') {
                write!(w, ".{}{}", m, self.inner)?;

            // For other selectors, scope it to be the children of the root element.
            } else {
                write!(w, ".{} {}", m, self.inner)?;
            }

        // For global styles, if it contains &, it will be replaced with html.
        } else if self.inner.contains('&') {
            write!(w, "{}", self.inner.replace("&", "html"))?;
        // For other styles, it will be written as is.
        } else {
            write!(w, "{}", self.inner)?;
        }

        Ok(())
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for Selector {
    fn from(s: T) -> Self {
        Self {
            inner: s.into(),
            kind: StringKind::Literal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_gen_simple() -> Result<()> {
        let s: Selector = ".abc".into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh .abc"
        );

        Ok(())
    }

    #[test]
    fn test_selector_pseduo() -> Result<()> {
        let s: Selector = ":hover".into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh:hover"
        );

        Ok(())
    }

    #[test]
    fn test_selector_root_pseduo() -> Result<()> {
        let s: Selector = ":root.big".into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh.big"
        );

        Ok(())
    }

    #[test]
    fn test_selector_gen_current() -> Result<()> {
        let s: Selector = "&.big".into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh.big"
        );

        Ok(())
    }
}
