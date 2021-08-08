use once_cell::sync::OnceCell;
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use crate::arch::{doc_head, document, JsValue};
use crate::ast::{Sheet, ToCss};
use crate::registry::{StyleKey, StyleRegistry};
use crate::utils::get_rand_str;

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
        let key = StyleKey(css);
        let reg = StyleRegistry::get_ref();
        let mut reg = reg.lock().unwrap();

        if let Some(m) = reg.get(&key) {
            return m.clone();
        }

        let new_style = Self {
            inner: Arc::new(StyleContent {
                class_name: format!("{}-{}", class_prefix, get_rand_str()),
                ast: key.0.clone(),
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

    /// Creates a new style
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist_core::{Style, ast::Sheet};
    ///
    /// let scopes: Sheet = Default::default();
    /// let style = Style::new_from_sheet(scopes);
    /// # Ok::<(), std::convert::Infallible>(())
    /// ```
    pub fn new_from_sheet(css: Sheet) -> Self {
        Self::create_from_sheet("stylist", css)
    }

    /// Creates a new style with custom class prefix
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist_core::Style;
    ///
    /// let scopes = Default::default();
    /// let style = Style::create_from_sheet("my-component", scopes);
    /// ```
    pub fn create_from_sheet<I: Borrow<str>>(class_prefix: I, css: Sheet) -> Self {
        Self::create_from_sheet_impl(class_prefix.borrow(), css)
    }

    /// Returns the class name for current style
    ///
    /// You can add this class name to the element to apply the style.
    ///
    /// # Examples
    ///
    /// ```
    /// use stylist_core::Style;
    ///
    /// let scopes = Default::default();
    /// let style = Style::create_from_sheet("stylist", scopes);
    ///
    /// // Example Output: stylist-uSu9NZZu
    /// println!("{}", style.get_class_name());
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
    /// use stylist_core::Style;
    ///
    /// let scopes = Default::default();
    /// let style = Style::create_from_sheet("my-component", scopes);
    ///
    /// // Example Output:
    /// // .my-component-uSu9NZZu {
    /// // color: red;
    /// // }
    /// println!("{}", style.get_style_str());
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
