use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{StringFragment, StyleContext, ToStyleStr};

/// A CSS Selector.
///
/// E.g.:
/// ```css
/// div[attr="val"].my-class#some-id
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Selector {
    pub fragments: Cow<'static, [StringFragment]>,
}

impl ToStyleStr for Selector {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        let mut joined_s = "".to_string();

        for frag in self.fragments.iter() {
            frag.write_style(&mut joined_s, ctx);
        }

        if let Some(ref m) = ctx.class_name {
            let scoped_class = format!(".{m}");
            // If contains current selector or root pseudo class, replace them with class name.
            if joined_s.contains('&') || joined_s.contains(":root") {
                w.push_str(
                    &joined_s
                        .replace('&', scoped_class.as_str())
                        .replace(":root", scoped_class.as_str()),
                );
            } else {
                w.push_str(&scoped_class);

                // If selector starts with a pseudo-class, apply it to the root element.
                // For other selectors, scope it to be the children of the root element.
                if !joined_s.starts_with(':') {
                    w.push(' ');
                }
                w.push_str(&joined_s);
            }

        // For global styles, if it contains &, it will be replaced with html.
        } else if joined_s.contains('&') {
            w.push_str(&joined_s.replace('&', ":root"));
        // For other styles, it will be written as is.
        } else {
            w.push_str(&joined_s);
        }
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
    fn test_selector_gen_simple() {
        let s: Selector = vec![".abc".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh")),
            ".stylist-abcdefgh .abc"
        );
    }

    #[test]
    fn test_selector_pseduo() {
        let s: Selector = vec![":hover".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh")),
            ".stylist-abcdefgh:hover"
        );
    }

    #[test]
    fn test_selector_root_pseduo() {
        let s: Selector = vec![":root.big".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh")),
            ".stylist-abcdefgh.big"
        );
    }

    #[test]
    fn test_selector_gen_current() {
        let s: Selector = vec!["&.big".into()].into();

        assert_eq!(
            s.to_style_str(Some("stylist-abcdefgh")),
            ".stylist-abcdefgh.big"
        );
    }
}
