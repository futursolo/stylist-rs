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
use stylist_core::ast::ToStyleStr;
use stylist_core::ResultDisplay;
use web_sys::Node;

mod content;
mod key;
mod registry;
#[cfg(feature = "ssr")]
mod ssr;
use crate::Result;
pub(crate) use content::StyleContent;
pub use key::StyleId;
pub(crate) use key::StyleKey;
use registry::StyleRegistry;

#[cfg(feature = "ssr")]
pub use ssr::*;

/// A builder for [`StyleManager`].
#[derive(Debug)]
pub struct StyleManagerBuilder {
    registry: RefCell<StyleRegistry>,

    prefix: Cow<'static, str>,
    container: Option<Node>,

    append: bool,

    #[cfg(feature = "ssr")]
    style_data: Option<std::sync::Arc<std::sync::Mutex<StyleData>>>,
}

impl Default for StyleManagerBuilder {
    fn default() -> Self {
        Self {
            registry: RefCell::default(),
            prefix: "stylist".into(),
            container: None,
            append: true,
            #[cfg(feature = "ssr")]
            style_data: None,
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
        #[cfg(not(feature = "not_browser_env"))]
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

    /// Returns the registry if it is availble, otherwise, creates the style and mounts it.
    pub(crate) fn get_or_register_style(&self, key: StyleKey) -> Result<Rc<StyleContent>> {
        let weak_mgr = self.downgrade();
        let mut reg = self.inner.registry.borrow_mut();

        if let Some(m) = reg.get(&key) {
            return Ok(m);
        }

        let id = match key.is_global {
            true => StyleId::new_global(&key.prefix),
            false => StyleId::new_scoped(&key.prefix),
        };

        // Non-global styles have ids prefixed in classes.
        let style_str = key.ast.to_style_str((!key.is_global).then_some(&id));

        // We parse the style str again in debug mode to ensure that interpolated values are
        // not corrupting the stylesheet.
        #[cfg(all(debug_assertions, feature = "debug_parser"))]
        style_str
            .parse::<crate::ast::Sheet>()
            .expect_display("debug: Stylist failed to parse the style with interpolated values");

        let content: Rc<_> = StyleContent {
            id,
            style_str,
            manager: weak_mgr,
            key: Rc::new(key),
        }
        .into();

        #[cfg(feature = "ssr")]
        {
            if let Some(ref mut style_data) = self.style_data() {
                // Automatically detach if has been used.
                style_data.as_vec_mut().push(StyleDataContent {
                    key: content.key().as_ref().clone(),
                    id: content.id().clone(),
                    style_str: content.get_style_str().to_string(),
                });

                // Register the created Style.
                reg.register(content.clone());

                return Ok(content);
            }
        }

        self.mount(&content)?;
        // Register the created Style.
        reg.register(content.clone());

        Ok(content)
    }

    pub(crate) fn unregister_style(&self, key: &Rc<StyleKey>) {
        self.inner.registry.borrow_mut().unregister(key);
    }

    /// Return a reference of style key.
    #[cfg(test)]
    fn get_registry(&self) -> &RefCell<StyleRegistry> {
        &self.inner.registry
    }

    /// Mount the [`Style`](crate::Style) into the DOM tree.
    #[cfg(all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(feature = "not_browser_env")
    ))]
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
    #[cfg(all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(feature = "not_browser_env")
    ))]
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
    #[cfg(any(
        not(target_arch = "wasm32"),
        target_os = "wasi",
        feature = "not_browser_env"
    ))]
    #[allow(unused_variables)]
    pub(crate) fn mount(&self, content: &StyleContent) -> Result<()> {
        // Does nothing on non-wasm targets.
        Ok(())
    }

    /// Unmount the [`Style`] from the DOM tree.
    #[cfg(any(
        not(target_arch = "wasm32"),
        target_os = "wasi",
        feature = "not_browser_env"
    ))]
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
    use std::sync::Arc;

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(super) struct StyleDataContent {
        pub key: StyleKey,
        pub id: StyleId,
        #[cfg(feature = "ssr")]
        #[serde(skip)]
        pub style_str: String,
    }

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
    #[derive(Debug, Clone)]
    pub struct StyleData(pub(super) Arc<Vec<StyleDataContent>>);

    impl Serialize for StyleData {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for StyleData {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Vec::<StyleDataContent>::deserialize(deserializer)
                .map(Arc::new)
                .map(Self)
        }
    }
}

#[cfg(any(feature = "ssr", feature = "hydration"))]
pub use feat_ssr_hydration::*;

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
            let mut reg = self.inner.registry.borrow_mut();

            for StyleDataContent { id, key, .. } in data.0.iter() {
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
