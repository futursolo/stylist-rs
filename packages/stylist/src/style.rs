use crate::{Error, Result};
use once_cell::sync::OnceCell;
use std::borrow::Cow;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use crate::arch::{doc_head, document, JsValue};
use crate::ast::{IntoSheet, Sheet, ToStyleStr};
use crate::registry::{StyleKey, StyleRegistry};
use crate::utils::get_entropy;

#[derive(Debug)]
struct StyleContent {
    key: StyleKey,

    /// The designated class name of this style
    class_name: String,

    style_str: OnceCell<String>,
}

impl StyleContent {
    fn get_class_name(&self) -> &str {
        &self.class_name
    }

    fn get_style_str(&self) -> &str {
        self.style_str
            .get_or_init(|| self.key.ast.to_style_str(self.get_class_name()))
    }

    /// Mounts the styles to the document
    #[cfg(target_arch = "wasm32")]
    fn mount(&self) -> std::result::Result<(), JsValue> {
        let document = document()?;
        let head = doc_head()?;

        let style_element = document.create_element("style")?;
        style_element.set_attribute("data-style", self.get_class_name())?;
        style_element.set_text_content(Some(self.get_style_str()));

        head.append_child(&style_element)?;
        Ok(())
    }

    /// Unmounts the style from the DOM tree
    /// Does nothing if it's not in the DOM tree
    #[cfg(target_arch = "wasm32")]
    fn unmount(&self) -> std::result::Result<(), JsValue> {
        let document = document()?;

        if let Some(m) =
            document.query_selector(&format!("style[data-style={}]", self.class_name))?
        {
            if let Some(parent) = m.parent_element() {
                parent.remove_child(&m)?;
            }
        }

        Ok(())
    }

    fn key(&self) -> &StyleKey {
        &self.key
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for StyleContent {
    /// Unmounts the style from the HTML head web-sys style
    fn drop(&mut self) {
        let _result = self.unmount();
    }
}

/// A struct that represents a scoped Style.
#[derive(Debug, Clone)]
pub struct Style {
    inner: Arc<StyleContent>,
}

impl Style {
    // The big method is monomorphic, so less code duplication and code bloat through generics
    // and inlining
    fn create_from_sheet_impl(class_prefix: Cow<'static, str>, css: Sheet) -> Result<Self> {
        let css = Arc::new(css);
        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey {
            prefix: class_prefix,
            ast: css,
        };
        let reg = StyleRegistry::get_ref();
        let mut reg = reg.lock().unwrap();

        if let Some(m) = reg.get(&key) {
            return Ok(m.clone());
        }

        let new_style = Self {
            inner: StyleContent {
                class_name: format!("{}-{}", key.prefix, get_entropy()),
                style_str: OnceCell::new(),
                key,
            }
            .into(),
        };

        #[cfg(target_arch = "wasm32")]
        new_style.inner.mount().map_err(|e| Error::Web(Some(e)))?;

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
    pub fn new<Css>(css: Css) -> Result<Self>
    where
        Css: IntoSheet,
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
    pub fn create<N, Css>(class_prefix: N, css: Css) -> Result<Self>
    where
        N: Into<Cow<'static, str>>,
        Css: IntoSheet,
    {
        let css = css.into_sheet()?;
        Self::create_from_sheet_impl(class_prefix.into(), css)
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
    pub(crate) fn key(&self) -> impl '_ + Deref<Target = StyleKey> {
        self.inner.key()
    }

    /// Unregister current style from style registry
    ///
    /// After calling this method, the style will be unmounted from DOM after all its clones are freed.
    pub fn unregister(&self) {
        let reg = StyleRegistry::get_ref();
        let mut reg = reg.lock().unwrap();
        reg.unregister(&*self.key());
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
"#,
                style_name = style.get_class_name()
            )
        )
    }
}
