//! Customise behaviour of Styles.
//!
//! This module contains [`StyleManager`] which can be used for customising
//! mounting point / mounting behaviour for styles (when rendering contents into a ShadowRoot or an <iframe />).
//!
//! This is an advanced feature and most of time you don't need to use it.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use once_cell::unsync::Lazy;
use web_sys::Node;

use crate::registry::StyleRegistry;
pub use crate::style::StyleId;
use crate::{Result, Style};

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

    /// Set the container [`Node`] for all style elements.
    pub fn container(mut self, value: Node) -> Self {
        self.container = Some(value);

        self
    }

    /// Build the StyleManager.
    pub fn build(self) -> Result<StyleManager> {
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

/// A struct to customise behaviour of [`Style`].
#[derive(Debug, Clone)]
pub struct StyleManager {
    inner: Rc<StyleManagerBuilder>,
}

impl StyleManager {
    /// Creates a builder for to build StyleManager.
    pub fn builder() -> StyleManagerBuilder {
        StyleManagerBuilder::new()
    }

    /// The default prefix used by the managed [`Style`] instances.
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

    /// Mount the [`Style`] into the DOM tree.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn mount(&self, style: &Style) -> Result<()> {
        use crate::arch::document;
        use crate::Error;

        let document = document()?;
        let container = self.container().ok_or(Error::Web(None))?;

        (|| {
            let style_element = document.create_element("style")?;
            style_element.set_attribute("data-style", style.id())?;
            style_element.set_text_content(Some(style.get_style_str()));

            container.append_child(&style_element)?;
            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }

    /// Unmount the [`Style`] from the DOM tree.
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
    pub(crate) fn mount(&self, style: &Style) -> Result<()> {
        Ok(())
    }

    /// Unmount the [`Style`] from the DOM tree.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unused_variables)]
    pub(crate) fn unmount(&self, id: &StyleId) -> Result<()> {
        Ok(())
    }
}

impl AsRef<StyleManager> for StyleManager {
    fn as_ref(&self) -> &StyleManager {
        self
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
