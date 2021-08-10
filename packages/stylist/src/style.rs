use crate::parser::Parser;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use crate::arch::{doc_head, document, JsValue};
use crate::registry::{StyleKey, StyleRegistry};
use crate::utils::get_rand_str;
use stylist_core::ast::{Sheet, ToCss};

#[derive(Debug)]
struct StyleContent {
    key: StyleKey,

    /// The designated class name of this style
    class_name: String,

    /// The abstract syntax tree of the css
    ast: Arc<Sheet>,

    style_str: OnceCell<String>,
}

impl StyleContent {
    fn get_class_name(&self) -> &str {
        &self.class_name
    }

    fn get_style_str(&self) -> &str {
        self.style_str
            .get_or_init(|| self.ast.to_css(self.get_class_name()))
    }

    /// Mounts the styles to the document
    #[cfg(target_arch = "wasm32")]
    fn mount(&self) -> Result<(), JsValue> {
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
    fn unmount(&self) -> Result<(), JsValue> {
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
    fn create_from_sheet_impl(class_prefix: &str, css: Sheet) -> Self {
        let css = Arc::new(css);
        // Creates the StyleKey, return from registry if already cached.
        let key = StyleKey(class_prefix.to_string(), css);
        let reg = StyleRegistry::get_ref();
        let mut reg = reg.lock().unwrap();

        if let Some(m) = reg.get(&key) {
            return m.clone();
        }

        let new_style = Self {
            inner: Arc::new(StyleContent {
                class_name: format!("{}-{}", class_prefix, get_rand_str()),
                ast: key.1.clone(),
                style_str: OnceCell::new(),
                key,
            }),
        };

        #[cfg(target_arch = "wasm32")]
        new_style.inner.mount().expect("Failed to mount");

        // Register the created Style.
        reg.register(new_style.clone());

        new_style
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
    pub fn new<Css: AsRef<str>>(css: Css) -> crate::Result<Self> {
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
    pub fn create<N: AsRef<str>, Css: AsRef<str>>(
        class_prefix: N,
        css: Css,
    ) -> crate::Result<Self> {
        let css = Parser::parse(css.as_ref())?;
        Ok(Style::create_from_sheet(class_prefix, css))
    }
    /// Creates a new style from an existing style sheet. Compared to [`Style::new`]
    /// the caller is responsible for generating the style, but the constructor is
    /// infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist::Style;
    /// use stylist_core::ast::Sheet;
    ///
    /// let scopes: Sheet = Default::default();
    /// let style = Style::new_from_sheet(scopes);
    /// ```
    pub fn new_from_sheet(css: Sheet) -> Self {
        Self::create_from_sheet("stylist", css)
    }
    /// Creates a new style from an existing style sheet and a custom class prefix.
    /// Compared to [`Style::create`] the caller is responsible for generating the style,
    /// but the constructor is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist::Style;
    ///
    /// let scopes = Default::default();
    /// let style = Style::create_from_sheet("my-component", scopes);
    /// ```
    pub fn create_from_sheet<I: AsRef<str>>(class_prefix: I, css: Sheet) -> Self {
        Self::create_from_sheet_impl(class_prefix.as_ref(), css)
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
    /// // Example Output: stylist-uSu9NZZu
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
    /// // color: red;
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
