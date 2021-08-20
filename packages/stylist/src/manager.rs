//! Customise behaviour of Styles.
//!
//! This module contains [`StyleManager`] which can be used for customising
//! mounting point / mounting behaviour for styles (when rendering contents into a `ShadowRoot` or an `<iframe />`).
//!
//! This is an advanced feature and most of time you don't need to use it.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use once_cell::unsync::Lazy;
use web_sys::Node;

use crate::registry::StyleRegistry;
use crate::style::StyleContent;
pub use crate::style::StyleId;
use crate::Result;

/// A builder for [`StyleManager`].
#[derive(Debug, Clone)]
pub struct StyleManagerBuilder {
    registry: Rc<RefCell<StyleRegistry>>,

    prefix: Cow<'static, str>,
    container: Option<Node>,

    append: bool,
}

impl Default for StyleManagerBuilder {
    fn default() -> Self {
        Self {
            registry: Rc::default(),
            prefix: "stylist".into(),
            container: None,
            append: true,
        }
    }
}

impl StyleManagerBuilder {
    /// Creates a builder for to build StyleManager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the prefix.
    pub fn prefix(mut self, value: Cow<'static, str>) -> Self {
        self.prefix = value;

        self
    }

    /// Set the container [`Node`] for all style elements managed by this manager.
    pub fn container(mut self, value: Node) -> Self {
        self.container = Some(value);

        self
    }

    /// Set the way how `<style />` tags are added to the container.
    ///
    /// When set to `false`, stylist will prepend the style tags to the container.
    ///
    /// Default: `true`
    pub fn append(mut self, value: bool) -> Self {
        self.append = value;

        self
    }

    /// Build the [`StyleManager`].
    #[allow(unused_mut)]
    pub fn build(mut self) -> Result<StyleManager> {
        #[cfg(target_arch = "wasm32")]
        if self.container.is_none() {
            use crate::arch::doc_head;
            self.container = Some(doc_head()?.into());
        }

        Ok(StyleManager {
            inner: Rc::new(self),
        })
    }
}

/// A struct to customise behaviour of [`Style`](crate::Style).
#[derive(Debug, Clone)]
pub struct StyleManager {
    inner: Rc<StyleManagerBuilder>,
}

impl StyleManager {
    /// Creates a builder for to build StyleManager.
    pub fn builder() -> StyleManagerBuilder {
        StyleManagerBuilder::new()
    }

    /// The default prefix used by the managed [`Style`](crate::Style) instances.
    pub fn prefix(&self) -> Cow<'static, str> {
        self.inner.prefix.clone()
    }

    /// The container [`Node`] for all style elements managed by this manager.
    pub fn container(&self) -> Option<Node> {
        self.inner.container.clone()
    }

    /// Get the Registry instance.
    pub(crate) fn get_registry(&self) -> Rc<RefCell<StyleRegistry>> {
        self.inner.registry.clone()
    }

    /// Mount the [`Style`](crate::Style) into the DOM tree.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn mount(&self, content: &StyleContent) -> Result<()> {
        use crate::arch::document;
        use crate::Error;

        let document = document()?;
        let container = self.container().ok_or(Error::Web(None))?;

        (|| {
            let style_element = document.create_element("style")?;
            style_element.set_attribute("data-style", content.id())?;
            style_element.set_text_content(Some(content.get_style_str()));

            // Prepend element
            if !self.inner.append {
                if let Some(m) = container.first_child() {
                    return m.insert_before(&style_element, Some(&m)).map(|_m| ());
                }
            }

            container.append_child(&style_element)?;
            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }

    /// Unmount the [`Style`](crate::Style) from the DOM tree.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn unmount(&self, id: &StyleId) -> Result<()> {
        use crate::arch::document;
        use crate::Error;

        let document = document()?;
        (|| {
            if let Some(m) = document.query_selector(&format!("style[data-style={}]", id))? {
                if let Some(parent) = m.parent_element() {
                    parent.remove_child(&m)?;
                }
            }

            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }

    /// Mount the [`Style`] in to the DOM tree.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unused_variables)]
    pub(crate) fn mount(&self, content: &StyleContent) -> Result<()> {
        // Does nothing on non-wasm targets.
        Ok(())
    }

    /// Unmount the [`Style`] from the DOM tree.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unused_variables)]
    pub(crate) fn unmount(&self, id: &StyleId) -> Result<()> {
        // Does nothing on non-wasm targets.
        Ok(())
    }
}

impl From<&Self> for StyleManager {
    fn from(m: &Self) -> Self {
        m.clone()
    }
}

impl Default for StyleManager {
    fn default() -> Self {
        thread_local! {
            static MGR: Lazy<StyleManager> = Lazy::new(|| StyleManager::builder().build().expect("Failed to create default manager."));
        }

        MGR.with(|m| (*m).clone())
    }
}
