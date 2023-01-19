use std::borrow::Cow;
use std::rc::Rc;

use crate::manager::{StyleContent, StyleId, StyleKey, StyleManager};
use crate::{Result, StyleSource};

/// A struct that represents a scoped Style.
///
/// # Style Scoping and Substitution Rule for Current Selector(`&`):
///
/// Currently, Stylist processes selectors with the following rules:
///
/// If a style attribute is not in any block (dangling style attribute), it will be scoped with
///   the generated class name. (Applied to the element where the generated class name is applied
///   to.)
///
///
///   For example, for the following style:
///
///   ```css
///   color: red;
///   ```
///
///   Stylist will generate the following stylesheet:
///
///   ```css
///   .stylist-uSu9NZZu {
///       color: red;
///   }
///   ```
///
///
/// For style attributes that are in a block, for each selector of that block, the following rules
///   apply:
/// - If a selector contains a Current Selector(`&`), the current selector will be substituted with
///   the generated class name.
/// - If a selector starts with a pseudo-class selector, it will be applied to the root element.
/// - For other selectors, it will be prefixed with the generated class name.
///
///   Example, original style:
///
///   ```css
///   &.invalid input, input:invalid {
///       color: red;
///   }
///
///   :hover {
///       background-color: pink;
///   }
///
///   .hint {
///       font-size: 0.9rem;
///   }
///   ```
///
///   Stylist generated stylesheet:
///
///   ```css
///   .stylist-uSu9NZZu.invalid input, .stylist-uSu9NZZu input:invalid {
///       color: red;
///   }
///
///   .stylist-uSu9NZZu:hover {
///       background-color: pink;
///   }
///
///   .stylist-uSu9NZZu .hint {
///       font-size: 0.9rem;
///   }
///   ```
///
///   ## Note:
///
///   Root pseudo class (`:root`) will also be treated like a Current Selector.
#[derive(Debug, Clone)]
pub struct Style {
    inner: Rc<StyleContent>,
}

impl Style {
    // The big method is monomorphic, so less code duplication and code bloat through generics
    // and inlining
    fn create_impl(
        class_prefix: Cow<'static, str>,
        css: StyleSource,
        manager: StyleManager,
    ) -> Result<Self> {
        let css = css.into_sheet();

        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey {
            is_global: false,
            prefix: class_prefix,
            ast: css,
        };

        let inner = manager.get_or_register_style(key)?;
        let new_style = Self { inner };

        Ok(new_style)
    }

    /// Creates a new style from some parsable css with a default prefix.
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
        Self::create(StyleManager::default().prefix(), css)
    }

    /// Creates a new style with a custom class prefix from some parsable css.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylist::Style;
    ///
    /// let style = Style::create("my-component", "background-color: red;")?;
    /// # Ok::<(), stylist::Error>(())
    /// ```
    pub fn create<N, Css>(class_prefix: N, css: Css) -> Result<Self>
    where
        N: Into<Cow<'static, str>>,
        Css: TryInto<StyleSource>,
        crate::Error: From<Css::Error>,
    {
        Self::create_with_manager(class_prefix, css, StyleManager::default())
    }

    /// Creates a new style from some parsable css with a default prefix using a custom
    /// manager.
    pub fn new_with_manager<Css, M>(css: Css, manager: M) -> Result<Self>
    where
        Css: TryInto<StyleSource>,
        crate::Error: From<Css::Error>,
        M: Into<StyleManager>,
    {
        let mgr = manager.into();
        Self::create_with_manager(mgr.prefix(), css, mgr.clone())
    }

    /// Creates a new style with a custom class prefix from some parsable css using a custom
    /// manager.
    pub fn create_with_manager<N, Css, M>(class_prefix: N, css: Css, manager: M) -> Result<Self>
    where
        N: Into<Cow<'static, str>>,
        Css: TryInto<StyleSource>,
        crate::Error: From<Css::Error>,
        M: Into<StyleManager>,
    {
        Self::create_impl(class_prefix.into(), css.try_into()?, manager.into())
    }

    /// Returns the class name for current style
    ///
    /// You can add this class name to the element to apply the style.
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist::Style;
    ///
    /// let style = Style::create("my-component", "background-color: red;")?;
    ///
    /// // Example Output: my-component-uSu9NZZu
    /// println!("{}", style.get_class_name());
    /// # Ok::<(), stylist::Error>(())
    /// ```
    pub fn get_class_name(&self) -> &str {
        self.inner.id()
    }

    /// Get the parsed and generated style in `&str`.
    ///
    /// This is usually used for debug purposes or testing in non-wasm32 targets.
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist::Style;
    ///
    /// let style = Style::create("my-component", "background-color: red;")?;
    ///
    /// // Example Output:
    /// // .my-component-uSu9NZZu {
    /// //     background-color: red;
    /// // }
    /// println!("{}", style.get_style_str());
    /// # Ok::<(), stylist::Error>(())
    /// ```
    pub fn get_style_str(&self) -> &str {
        self.inner.get_style_str()
    }

    /// Return a reference of style key.
    #[cfg(test)]
    pub(crate) fn key(&self) -> &Rc<StyleKey> {
        self.inner.key()
    }

    /// Unregister current style from style registry.
    ///
    /// After calling this method, the style will be unmounted from DOM after all its clones are
    /// freed.
    ///
    /// # Note
    ///
    /// Most of time, you don't need to unmount a style.
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
        Style::new("background-color: black;").expect("Failed to create Style.");
    }

    #[test]
    fn test_complex() {
        let style = Style::new(
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

                    @supports (max-width: 500px) {
                        @media screen and (max-width: 500px) {
                            display: flex;
                        }
                    }
                }
            "#,
        )
        .expect("Failed to create Style.");

        assert_eq!(
            style.get_style_str(),
            format!(
                r#".{style_name} {{
    background-color: black;
}}
.{style_name} .with-class {{
    color: red;
}}
@media screen and (max-width: 600px) {{
    .{style_name} {{
        color: yellow;
    }}
}}
@supports (display: grid) {{
    .{style_name} {{
        display: grid;
    }}
}}
.{style_name} header, .{style_name} footer {{
    border: 1px solid black;
}}
@supports (max-width: 500px) {{
    @media screen and (max-width: 500px) {{
        .{style_name} header, .{style_name} footer {{
            display: flex;
        }}
    }}
}}
"#,
                style_name = style.get_class_name()
            )
        )
    }
}
