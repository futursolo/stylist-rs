use std::{borrow::Cow, fmt};

use super::{StringFragment, ToStyleStr};
use crate::Result;

/// A CSS Selector.
///
/// E.g.:
/// ```css
/// div[attr="val"].my-class#some-id
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Selector {
    pub fragments: Cow<'static, [StringFragment]>,
}

impl ToStyleStr for Selector {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> Result<()> {
        let mut joined_s = "".to_string();

        for frag in self.fragments.iter() {
            frag.write_style(&mut joined_s, class_name)?;
        }

        if let Some(m) = class_name {
            // If contains current selector or root pseudo class, replace them with class name.
            if joined_s.contains('&') || joined_s.contains(":root") {
                let scoped_class = format!(".{}", m);

                write!(
                    w,
                    "{}",
                    joined_s
                        .replace("&", scoped_class.as_str())
                        .replace(":root", scoped_class.as_str())
                )?;

            // If selector starts with a pseudo-class, apply it to the root element.
            } else if joined_s.starts_with(':') {
                write!(w, ".{}{}", m, joined_s)?;

            // For other selectors, scope it to be the children of the root element.
            } else {
                write!(w, ".{} {}", m, joined_s)?;
            }

        // For global styles, if it contains &, it will be replaced with html.
        } else if joined_s.contains('&') {
            write!(w, "{}", joined_s.replace("&", "html"))?;
        // For other styles, it will be written as is.
        } else {
            write!(w, "{}", joined_s)?;
        }

        Ok(())
    }
}

impl<T: Into<Cow<'static, [StringFragment]>>> From<T> for Selector {
    fn from(s: T) -> Self {
        Self {
            fragments: s.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_gen_simple() -> Result<()> {
        let s: Selector = vec![".abc".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh .abc"
        );

        Ok(())
    }

    #[test]
    fn test_selector_pseduo() -> Result<()> {
        let s: Selector = vec![":hover".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh:hover"
        );

        Ok(())
    }

    #[test]
    fn test_selector_root_pseduo() -> Result<()> {
        let s: Selector = vec![":root.big".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh.big"
        );

        Ok(())
    }

    #[test]
    fn test_selector_gen_current() -> Result<()> {
        let s: Selector = vec!["&.big".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh"))?,
            ".stylist-abcdefgh.big"
        );

        Ok(())
    }
}
