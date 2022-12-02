//! Customise behaviour of Styles.
//!
//! This module contains [`StyleManager`] which can be used for customising
//! mounting point / mounting behaviour for styles (when rendering contents into a `ShadowRoot` or
//! an `<iframe />`).
//!
//! This is an advanced feature and most of the time you don't need to use it.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use once_cell::unsync::Lazy;
use stylist_core::ResultDisplay;
use web_sys::Node;

use crate::registry::StyleRegistry;
use crate::style::StyleContent;
pub use crate::style::StyleId;
use crate::Result;

/// A builder for [`StyleManager`].
#[derive(Debug)]
pub struct StyleManagerBuilder {
    registry: RefCell<StyleRegistry>,

    prefix: Cow<'static, str>,
    container: Option<Node>,

    append: bool,
}

impl Default for StyleManagerBuilder {
    fn default() -> Self {
        Self {
            registry: RefCell::default(),
            prefix: "stylist".into(),
            container: None,
            append: true,
        }
    }
}
impl PartialEq for StyleManager {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
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

// A style manager, but with a weak reference.
#[derive(Debug, Clone)]
pub(crate) struct WeakStyleManager {
    inner: Weak<StyleManagerBuilder>,
}

impl WeakStyleManager {
    pub fn upgrade(&self) -> Option<StyleManager> {
        self.inner.upgrade().map(|inner| StyleManager { inner })
    }
}

/// A struct to customise behaviour of [`Style`](crate::Style).
#[derive(Debug, Clone)]
pub struct StyleManager {
    inner: Rc<StyleManagerBuilder>,
}

impl StyleManager {
    /// Create a new StyleManager with default configuration.
    pub fn new() -> Result<StyleManager> {
        Self::builder().build()
    }

    /// Creates a builder for to build StyleManager.
    pub fn builder() -> StyleManagerBuilder {
        StyleManagerBuilder::new()
    }

    pub(crate) fn downgrade(&self) -> WeakStyleManager {
        WeakStyleManager {
            inner: Rc::downgrade(&self.inner),
        }
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
    pub(crate) fn get_registry(&self) -> &RefCell<StyleRegistry> {
        &self.inner.registry
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
    pub(crate) fn unmount(id: &StyleId) -> Result<()> {
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
    pub(crate) fn unmount(id: &StyleId) -> Result<()> {
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
            static MGR: Lazy<StyleManager> = Lazy::new(|| StyleManager::builder().build().expect_display("Failed to create default manager."));
        }

        MGR.with(|m| (*m).clone())
    }
}

#[cfg(any(feature = "ssr", feature = "hydration"))]
mod feat_ssr_hydration {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::registry::StyleKey;

    /// Data of Styles managed by the current style manager.
    ///
    /// This type is serializable and deserializable.
    /// This type should be sent to the client side and loaded with
    /// [`StyleManager::load_style_data`].
    ///
    /// # Note
    ///
    /// If you are using [`ManagerProvider`](crate::yew::ManagerProvider),
    /// this behaviour is managed automatically.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StyleData(pub(super) Vec<(StyleKey, StyleId)>);
}

#[cfg(any(feature = "ssr", feature = "hydration"))]
pub use feat_ssr_hydration::*;

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt;

    use super::*;

    impl StyleManager {
        /// Returns StyleData of current style manager.
        ///
        /// # Note
        ///
        /// If you are using [`ManagerProvider`](crate::yew::ManagerProvider),
        /// this behaviour is managed automatically.
        pub fn style_data(&self) -> StyleData {
            let reg = self.get_registry();
            let reg = reg.borrow();

            StyleData(
                reg.styles
                    .iter()
                    .map(|(key, v)| ((**key).clone(), v.id().to_owned()))
                    .collect::<Vec<_>>(),
            )
        }

        /// Writes styles stored in the manager as `<style data-style="stylist-...">...</style>`.
        pub fn write_static<W>(&self, w: &mut W) -> fmt::Result
        where
            W: fmt::Write,
        {
            let reg = self.get_registry();
            let reg = reg.borrow();

            for content in reg.styles.values() {
                // We cannot guarantee a valid class name if the user choose to use a custom prefix.
                // If the default prefix is used, StyleId is guaranteed to be valid without
                // escaping.
                write!(w, r#"<style data-style="{}">"#, content.id())?;
                write!(w, "{}", html_escape::encode_style(content.get_style_str()))?;
                write!(w, "</style>")?;
            }

            Ok(())
        }
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use crate::ast::ToStyleStr;
    use std::collections::hash_map::Entry;

    impl StyleManager {
        /// Loads StyleData of current style manager.
        ///
        /// # Note
        ///
        /// If you are using [`ManagerProvider`](crate::yew::ManagerProvider),
        /// this behaviour is managed automatically.
        ///
        /// # Panics
        ///
        /// This method should be called as early as possible.
        /// If the same style to be loaded already existed in the manager, it will panic.
        pub fn load_style_data(&self, data: &StyleData) {
            let reg = self.get_registry();
            let mut reg = reg.borrow_mut();

            for (key, id) in data.0.iter() {
                let key = Rc::new(key.clone());

                match reg.styles.entry(key.clone()) {
                    Entry::Occupied(m) => {
                        assert_eq!(
                            m.get().id(),
                            id,
                            "An existing style has been rendered with a different class, this is not supported, please load style data first!"
                        );
                    }
                    Entry::Vacant(m) => {
                        m.insert(
                            StyleContent {
                                is_global: key.is_global,
                                id: id.clone(),
                                style_str: key.ast.to_style_str(Some(id)),
                                manager: self.downgrade(),
                                key: key.clone(),
                            }
                            .into(),
                        );
                    }
                }
            }
        }
    }
}
