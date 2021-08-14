use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use once_cell::unsync::{Lazy, OnceCell};
use web_sys::Node;

use crate::arch::doc_head;
use crate::registry::StyleRegistry;
pub use crate::style::StyleId;
use crate::{Result, Style};

/// A trait to customise behaviour of [`Style`].
///
/// This is an advanced trait and is mostly used for customising mounting point /
/// mounting behaviour for styles (when rendering contents into a `ShadowRoot` or an `<iframe />`).
pub trait StyleManager: fmt::Debug {
    /// The default prefix used by the managed [`Style`] instances.
    fn prefix(&self) -> Cow<'static, str> {
        "stylist".into()
    }

    /// Returns an [`Arc<Mutex<StyleRegistry>>`] of [`StyleRegistry`].
    fn get_registry(&self) -> Rc<RefCell<StyleRegistry>>;

    /// Returns the container element of all styles managed by this StyleManager.
    /// By default, this method returns the `<head />` element.
    fn get_container(&self) -> Result<Node> {
        let head = doc_head()?;
        Ok(head.into())
    }

    /// Mount the [`Style`] into the DOM tree.
    #[cfg(target_arch = "wasm32")]
    fn mount(&self, style: &Style) -> Result<()> {
        use crate::arch::document;
        use crate::Error;

        let document = document()?;
        let container = self.get_container()?;

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
    fn unmount(&self, id: &StyleId) -> Result<()> {
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
    fn mount(&self, style: &Style) -> Result<()> {
        Ok(())
    }

    /// Unmount the [`Style`] from the DOM tree.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unused_variables)]
    fn unmount(&self, id: &StyleId) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
struct ManagerInner {
    registry: Rc<RefCell<StyleRegistry>>,
    container: OnceCell<Node>,
}

/// The default Style Manager.
#[derive(Debug)]
pub(crate) struct DefaultManager {
    inner: Rc<ManagerInner>,
}

impl StyleManager for DefaultManager {
    fn get_registry(&self) -> Rc<RefCell<StyleRegistry>> {
        self.inner.registry.clone()
    }

    fn get_container(&self) -> Result<Node> {
        self.inner
            .container
            .get_or_try_init(|| {
                let head = doc_head()?;
                Ok(head.into())
            })
            .map(|m| m.clone())
    }
}

impl Default for DefaultManager {
    fn default() -> Self {
        thread_local! {
            static MGR_INNER: Lazy<Rc<ManagerInner>> = Lazy::new(|| {
                Rc::default()
            });
        }

        DefaultManager {
            inner: MGR_INNER.with(|m| (*m).clone()),
        }
    }
}
