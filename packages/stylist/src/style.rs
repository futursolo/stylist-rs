use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[cfg(all(debug_assertions, feature = "parser"))]
use stylist_core::ResultDisplay;

use crate::ast::ToStyleStr;
use crate::manager::StyleManager;
use crate::registry::StyleKey;
use crate::{Result, StyleSource};

use crate::utils::get_entropy;

/// The Unique Identifier of a Style.
///
/// This is primarily used by [`StyleManager`] to track the mounted instance of [`Style`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleId(pub(crate) String);

impl Deref for StyleId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl fmt::Display for StyleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) struct StyleContent {
    #[allow(dead_code)]
    pub is_global: bool,

    pub id: StyleId,

    pub key: Rc<StyleKey>,

    pub style_str: String,

    pub manager: StyleManager,
}

impl StyleContent {
    pub fn id(&self) -> &StyleId {
        &self.id
    }

    pub fn get_style_str(&self) -> &str {
        &self.style_str
    }

    pub fn unmount(&self) -> Result<()> {
        self.manager().unmount(self.id())
    }

    pub fn key(&self) -> Rc<StyleKey> {
        self.key.clone()
    }

    pub fn manager(&self) -> &StyleManager {
        &self.manager
    }
}

impl Drop for StyleContent {
    /// Unmounts the style from the HTML head web-sys style
    fn drop(&mut self) {
        let _result = self.unmount();
    }
}

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
        css: StyleSource<'_>,
        manager: StyleManager,
    ) -> Result<Self> {
        #[cfg(all(debug_assertions, feature = "parser"))]
        use crate::ast::Sheet;

        let css = css.try_into_sheet()?;

        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey {
            is_global: false,
            prefix: class_prefix,
            ast: css,
        };

        let reg = manager.get_registry();
        let mut reg = reg.borrow_mut();

        if let Some(m) = reg.get(&key) {
            return Ok(Style { inner: m });
        }

        let id = StyleId(format!("{}-{}", key.prefix, get_entropy()));

        let style_str = key.ast.to_style_str(Some(&id));

        // We parse the style str again in debug mode to ensure that interpolated values are
        // not corrupting the stylesheet.
        #[cfg(all(debug_assertions, feature = "parser"))]
        style_str
            .parse::<Sheet>()
            .expect_display("debug: Stylist failed to parse the style with interpolated values");

        let new_style = Self {
            inner: StyleContent {
                is_global: false,
                id,
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
    pub fn new<'a, Css>(css: Css) -> Result<Self>
    where
        Css: Into<StyleSource<'a>>,
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
    pub fn create<'a, N, Css>(class_prefix: N, css: Css) -> Result<Self>
    where
        N: Into<Cow<'static, str>>,
        Css: Into<StyleSource<'a>>,
    {
        Self::create_impl(class_prefix.into(), css.into(), StyleManager::default())
    }

    /// Creates a new style from some parsable css with a default prefix using a custom
    /// manager.
    pub fn new_with_manager<'a, Css, M>(css: Css, manager: M) -> Result<Self>
    where
        Css: Into<StyleSource<'a>>,
        M: Into<StyleManager>,
    {
        let mgr = manager.into();
        Self::create_impl(mgr.prefix(), css.into(), mgr.clone())
    }

    /// Creates a new style with a custom class prefix from some parsable css using a custom
    /// manager.
    pub fn create_with_manager<'a, N, Css, M>(class_prefix: N, css: Css, manager: M) -> Result<Self>
    where
        N: Into<Cow<'static, str>>,
        Css: Into<StyleSource<'a>>,
        M: Into<StyleManager>,
    {
        Self::create_impl(class_prefix.into(), css.into(), manager.into())
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
    pub(crate) fn key(&self) -> Rc<StyleKey> {
        self.inner.key()
    }

    /// Unregister current style from style registry.
    ///
    /// After calling this method, the style will be unmounted from DOM after all its clones are freed.
    ///
    /// # Note
    ///
    /// Most of time, you don't need to unmount a style.
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
