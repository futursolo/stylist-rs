use std::rc::Rc;

use crate::manager::{StyleContent, StyleId, StyleKey, StyleManager};
use crate::{Result, StyleSource};

/// A struct that represents a global Style.
///
/// This class is equivalent to [`Style`](crate::Style) but for global styles.
///
/// It will replace Current Selectors (`&`) with `:root` and apply dangling style attributes to
/// the root element (`html` when style is not applied in a Shadow DOM).
#[derive(Debug, Clone)]
pub struct GlobalStyle {
    inner: Rc<StyleContent>,
}

impl GlobalStyle {
    // The big method is monomorphic, so less code duplication and code bloat through generics
    // and inlining
    fn create_impl(css: StyleSource, manager: StyleManager) -> Result<Self> {
        let css = css.into_sheet();

        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey {
            is_global: true,
            prefix: manager.prefix(),
            ast: css,
        };

        let inner = manager.get_or_register_style(key)?;
        let new_style = Self { inner };

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
    pub fn new<Css>(css: Css) -> Result<Self>
    where
        Css: TryInto<StyleSource>,
        crate::Error: From<Css::Error>,
    {
        Self::new_with_manager(css, StyleManager::default())
    }

    /// Creates a new style using a custom manager.
    pub fn new_with_manager<Css, M>(css: Css, manager: M) -> Result<Self>
    where
        Css: TryInto<StyleSource>,
        crate::Error: From<Css::Error>,
        M: Into<StyleManager>,
    {
        let mgr = manager.into();
        Self::create_impl(css.try_into()?, mgr)
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

    /// Unregister current style from style registry.
    ///
    /// After calling this method, the style will be unmounted from DOM after all its clones are
    /// freed.
    pub fn unregister(&self) {
        self.inner.unregister();
    }

    /// Returns the [`StyleId`] for current style.
    pub fn id(&self) -> &StyleId {
        self.inner.id()
    }
}

#[cfg(test)]
#[cfg(feature = "parser")]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let global_style =
            GlobalStyle::new("background-color: black;").expect("Failed to create Style.");
        assert_eq!(
            global_style.get_style_str(),
            r#":root {
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
            r#":root {
    background-color: black;
}
.with-class {
    color: red;
}
@media screen and (max-width: 600px) {
    :root {
        color: yellow;
    }
}
@supports (display: grid) {
    :root {
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
