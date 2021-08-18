use once_cell::unsync::OnceCell;
use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

use crate::ast::{IntoSheet, Sheet, ToStyleStr};
use crate::manager::StyleManager;
use crate::registry::StyleKey;
use crate::{Error, Result};

use crate::utils::get_entropy;

/// The Unique Identifier of a Style.
///
/// This is currently the same as the class name of a style.
/// But this may change in the future.
///
/// This is primarily used by [`StyleManager`] to track the mounted instance of [`Style`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleId(String);

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
struct StyleContent {
    key: Rc<StyleKey<'static>>,

    /// The designated class name of this style
    class_name: String,

    style_str: OnceCell<String>,

    id: OnceCell<StyleId>,

    manager: StyleManager,
}

impl StyleContent {
    fn id(&self) -> &StyleId {
        self.id
            .get_or_init(|| StyleId(self.get_class_name().to_owned()))
    }

    fn get_class_name(&self) -> &str {
        &self.class_name
    }

    fn get_style_str(&self) -> &str {
        self.style_str
            .get_or_init(|| self.key.ast.to_style_str(self.get_class_name()))
    }

    fn unmount(&self) -> Result<()> {
        self.manager().unmount(self.id())
    }

    fn key(&self) -> Rc<StyleKey<'static>> {
        self.key.clone()
    }

    fn manager(&self) -> &StyleManager {
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
/// Currently, Stylist processes selectors follow the following rules:
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
/// - If a selector does not contain a Current Selector, it will be prefixed with the generated
///   class name.
///
///
///   Example, original style:
///
///   ```css
///   &.invalid input, input:invalid {
///       color: red;
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
///   .stylist-uSu9NZZu .hint {
///       font-size: 0.9rem;
///   }
///   ```
#[derive(Debug, Clone)]
pub struct Style {
    inner: Rc<StyleContent>,
}

impl Style {
    // The big method is monomorphic, so less code duplication and code bloat through generics
    // and inlining
    fn create_impl(
        class_prefix: Cow<'static, str>,
        css: Cow<'_, Sheet>,
        manager: StyleManager,
    ) -> Result<Self> {
        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey {
            prefix: class_prefix,
            ast: css,
        };

        let reg = manager.get_registry();
        let mut reg = reg.borrow_mut();

        if let Some(m) = reg.get(&key) {
            return Ok(m);
        }

        let key = StyleKey {
            prefix: key.prefix,
            // I don't think there's a way to turn a Cow<'_, Sheet> to a Cow<'static, Sheet>
            // if inner is &'static Sheet without cloning.
            // But I think it would be good enough if allocation only happens once.
            ast: Cow::Owned(key.ast.into_owned()),
        };

        let new_style = Self {
            inner: StyleContent {
                class_name: format!("{}-{}", key.prefix, get_entropy()),
                style_str: OnceCell::new(),
                id: OnceCell::new(),
                manager,
                key: Rc::new(key),
            }
            .into(),
        };

        new_style.inner.manager().mount(&new_style)?;

        // Register the created Style.
        reg.register(new_style.clone());

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
        Css: IntoSheet<'a>,
    {
        Self::create("stylist", css)
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
        Css: IntoSheet<'a>,
    {
        let css = css.into_sheet()?;
        Self::create_impl(class_prefix.into(), css, StyleManager::default())
    }

    /// Creates a new style from some parsable css with a default prefix using a custom
    /// manager.
    #[cfg_attr(documenting, doc(cfg(feature = "style_manager")))]
    pub fn new_with_manager<'a, Css, M>(css: Css, manager: M) -> Result<Self>
    where
        Css: IntoSheet<'a>,
        M: AsRef<StyleManager>,
    {
        let css = css.into_sheet()?;
        let mgr = manager.as_ref();
        Self::create_impl(mgr.prefix(), css, mgr.clone())
    }

    /// Creates a new style with a custom class prefix from some parsable css using a custom
    /// manager.
    #[cfg_attr(documenting, doc(cfg(feature = "style_manager")))]
    pub fn create_with_manager<'a, N, Css, M>(class_prefix: N, css: Css, manager: M) -> Result<Self>
    where
        N: Into<Cow<'static, str>>,
        Css: IntoSheet<'a>,
        M: AsRef<StyleManager>,
    {
        let css = css.into_sheet()?;
        Self::create_impl(class_prefix.into(), css, manager.as_ref().clone())
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
        self.inner.get_class_name()
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
    /// // background-color: red;
    /// // }
    /// println!("{}", style.get_style_str());
    /// # Ok::<(), stylist::Error>(())
    /// ```
    pub fn get_style_str(&self) -> &str {
        self.inner.get_style_str()
    }

    /// Return a reference of style key.
    pub(crate) fn key(&self) -> Rc<StyleKey<'static>> {
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

impl FromStr for Style {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Style::new(s)
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
"#,
                style_name = style.get_class_name()
            )
        )
    }
}
