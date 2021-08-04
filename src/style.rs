use once_cell::sync::OnceCell;
use std::borrow::Cow;
use std::sync::Arc;

use crate::ast::{Scope, ToCss};
use crate::parser::Parser;
use crate::registry::{StyleKey, StyleRegistry};
use crate::utils::get_rand_str;
#[cfg(target_arch = "wasm32")]
use crate::utils::{doc_head, document};
#[cfg(target_arch = "wasm32")]
use crate::Error;
use crate::Result;

#[derive(Debug)]
struct StyleContent {
    key: Arc<StyleKey>,

    /// The designated class name of this style
    class_name: String,

    /// The abstract syntax tree of the css
    ast: Arc<Vec<Scope>>,

    style_str: OnceCell<String>,
}

impl StyleContent {
    fn get_class_name(&self) -> &str {
        &self.class_name
    }

    fn get_style_str(&self) -> &str {
        self.style_str.get_or_init(|| {
            self.ast
                .iter()
                .map(|scope| scope.to_css(self.get_class_name()))
                .fold(String::new(), |mut acc, css_part| {
                    acc.push('\n');
                    acc.push_str(&css_part);
                    acc
                })
        })
    }

    /// Mounts the styles to the document
    #[cfg(target_arch = "wasm32")]
    fn mount(&self) -> Result<()> {
        let document = document()?;
        let head = doc_head()?;

        let style_element = document
            .create_element("style")
            .map_err(|e| Error::Web(Some(e)))?;
        style_element
            .set_attribute("data-style", self.get_class_name())
            .map_err(|e| Error::Web(Some(e)))?;
        style_element.set_text_content(Some(self.get_style_str()));

        head.append_child(&style_element)
            .map_err(|e| Error::Web(Some(e)))?;
        Ok(())
    }

    /// Unmounts the style from the DOM tree
    /// Does nothing if it's not in the DOM tree
    #[cfg(target_arch = "wasm32")]
    fn unmount(&self) -> Result<()> {
        let document = document()?;

        if let Some(m) = document
            .query_selector(&format!("style[data-style={}]", self.class_name))
            .map_err(|e| Error::Web(Some(e)))?
        {
            if let Some(parent) = m.parent_element() {
                parent.remove_child(&m).map_err(|e| Error::Web(Some(e)))?;
            }
        }

        Ok(())
    }

    fn key(&self) -> Arc<StyleKey> {
        self.key.clone()
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
    /// Creates a new style
    ///
    /// # Examples
    ///
    /// ```
    /// let style = Style::new("color:red").expect("Failed to create style.");
    /// ```
    pub fn new<S: Into<Cow<'static, str>>>(css: S) -> Result<Self> {
        Self::create("stylist", css)
    }

    /// Returns the class name for current style
    ///
    /// You can add this class name to the element to apply the style.
    ///
    /// # Examples
    ///
    /// ```
    /// let style = Style::new("color:red").expect("Failed to create style.");
    ///
    /// // Example Output: stylist-uSu9NZZu
    /// println!("{}", style.get_class_name());
    /// ```
    pub fn get_class_name(&self) -> &str {
        self.inner.get_class_name()
    }

    fn create_impl(key: StyleKey) -> Result<Self> {
        let ast = Parser::parse(&*key.1)?;
        let new_style = Self {
            inner: Arc::new(StyleContent {
                class_name: format!("{}-{}", key.0, get_rand_str()),
                ast: Arc::new(ast),
                style_str: OnceCell::new(),
                key: Arc::new(key),
            }),
        };

        #[cfg(target_arch = "wasm32")]
        new_style.inner.mount()?;

        Ok(new_style)
    }

    /// Creates a new style with custom class prefix
    ///
    /// # Examples
    ///
    /// ```
    /// let style = Style::create("my-component", "color:red").expect("Failed to create style.");
    /// ```
    pub fn create<I1: Into<String>, I2: Into<Cow<'static, str>>>(
        class_prefix: I1,
        css: I2,
    ) -> Result<Self> {
        let (class_prefix, css) = (class_prefix.into(), css.into());

        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey(class_prefix, css.clone());
        if let Some(m) = StyleRegistry::get(&key) {
            return Ok(m);
        }

        let new_style = Self::create_impl(key)?;

        // Register the created Style.
        StyleRegistry::register(new_style.clone());

        Ok(new_style)
    }

    /// Get the parsed and generated style in `&str`.
    ///
    /// This is usually used for debug purposes or testing in non-wasm32 targets.
    ///
    /// # Examples
    ///
    /// ```
    /// let style = Style::create("my-component", "color:red").expect("Failed to create style.");
    ///
    /// // Example Output:
    /// // .stylist-uSu9NZZu {
    /// // color: red;
    /// // }
    /// println!("{}", style.get_style_str());
    /// ```
    pub fn get_style_str(&self) -> &str {
        self.inner.get_style_str()
    }

    /// Return a reference of style key.
    pub(crate) fn key(&self) -> Arc<StyleKey> {
        self.inner.key()
    }

    /// Unregister current style from style registry
    ///
    /// After calling this method, the style will be unmounted from DOM after all its clones are freed.
    pub fn unregister(&self) {
        StyleRegistry::unregister(&*self.key());
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
                r#"
.{style_name} {{
background-color: black;
}}
.{style_name} .with-class {{
color: red;
}}

@media screen and (max-width: 600px) {{
.{style_name} {{
color: yellow;
}}
}}"#,
                style_name = style.get_class_name()
            )
        )
    }
}
