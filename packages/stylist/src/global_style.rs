use std::rc::Rc;

#[cfg(all(debug_assertions, feature = "parser"))]
use stylist_core::ResultDisplay;

use crate::ast::ToStyleStr;
use crate::manager::StyleManager;
use crate::registry::StyleKey;
use crate::style::StyleContent;
use crate::style::StyleId;
use crate::utils::get_entropy;
use crate::{Result, StyleSource};

/// A struct that represents a global Style.
///
/// This class is equivalent to [`Style`](crate::Style) but for global styles.
///
/// It will replace Current Selectors (`&`) with `html` and apply dangling style attributes to
/// html.
#[derive(Debug, Clone)]
pub struct GlobalStyle {
    inner: Rc<StyleContent>,
}

impl GlobalStyle {
    // The big method is monomorphic, so less code duplication and code bloat through generics
    // and inlining
    fn create_impl(css: StyleSource<'_>, manager: StyleManager) -> Result<Self> {
        #[cfg(all(debug_assertions, feature = "parser"))]
        use crate::ast::Sheet;

        let prefix = format!("{}-global", manager.prefix());
        let css = css.try_to_sheet()?;

        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey {
            is_global: true,
            prefix: prefix.into(),
            ast: css,
        };

        let reg = manager.get_registry();
        let mut reg = reg.borrow_mut();

        if let Some(m) = reg.get(&key) {
            return Ok(Self { inner: m });
        }

        let style_str = key.ast.to_style_str(None);

        // We parse the style str again in debug mode to ensure that interpolated values are
        // not corrupting the stylesheet.
        #[cfg(all(debug_assertions, feature = "parser"))]
        style_str
            .parse::<Sheet>()
            .expect_display("debug: Stylist failed to parse the style with interpolated values");

        let new_style = Self {
            inner: StyleContent {
                is_global: true,
                id: StyleId(format!("{}-{}", key.prefix, get_entropy())),
                style_str,
                manager,
                key: Rc::new(key),
            }
            .into(),
        };

        new_style.inner.manager().mount(&new_style.inner)?;

        // Register the created Style.
        reg.register(new_style.inner.clone());

        Ok(new_style)
    }

    /// Creates a new style from some parsable css.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylist::Style;
    ///
    /// let style = Style::new("background-color: red;")?;
    /// # Ok::<(), stylist::Error>(())
    /// ```
    pub fn new<'a, Css>(css: Css) -> Result<Self>
    where
        Css: Into<StyleSource<'a>>,
    {
        Self::new_with_manager(css, StyleManager::default())
    }

    /// Creates a new style using a custom manager.
    pub fn new_with_manager<'a, Css, M>(css: Css, manager: M) -> Result<Self>
    where
        Css: Into<StyleSource<'a>>,
        M: Into<StyleManager>,
    {
        let mgr = manager.into();
        Self::create_impl(css.into(), mgr)
    }

    /// Get the parsed and generated style in `&str`.
    ///
    /// This is usually used for debug purposes or testing in non-wasm32 targets.
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist::GlobalStyle;
    ///
    /// let style = GlobalStyle::new("background-color: red;")?;
    ///
    /// // Example Output:
    /// // html {
    /// // background-color: red;
    /// // }
    /// println!("{}", style.get_style_str());
    /// # Ok::<(), stylist::Error>(())
    /// ```
    pub fn get_style_str(&self) -> &str {
        self.inner.get_style_str()
    }

    /// Returns a reference of style key.
    pub(crate) fn key(&self) -> Rc<StyleKey> {
        self.inner.key()
    }

    /// Unregister current style from style registry.
    ///
    /// After calling this method, the style will be unmounted from DOM after all its clones are freed.
    pub fn unregister(&self) {
        let reg = self.inner.manager().get_registry();
        let mut reg = reg.borrow_mut();
        reg.unregister(self.key());
    }

    /// Returns the [`StyleId`] for current style.
    pub fn id(&self) -> &StyleId {
        self.inner.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let global_style =
            GlobalStyle::new("background-color: black;").expect("Failed to create Style.");
        assert_eq!(
            global_style.get_style_str(),
            r#"html {
    background-color: black;
}
"#
        );
    }

    #[test]
    fn test_complex() {
        let global_style = GlobalStyle::new(
            r#"
                background-color: black;
                .with-class {
                    color: red;
                }
                @media screen and (max-width: 600px) {
                    color: yellow;
                }
                @supports (display: grid) {
                    display: grid;
                }

                header, footer {
                    border: 1px solid black;
                }
            "#,
        )
        .expect("Failed to create Style.");

        assert_eq!(
            global_style.get_style_str(),
            r#"html {
    background-color: black;
}
.with-class {
    color: red;
}
@media screen and (max-width: 600px) {
    html {
        color: yellow;
    }
}
@supports (display: grid) {
    html {
        display: grid;
    }
}
header, footer {
    border: 1px solid black;
}
"#,
        )
    }
}
