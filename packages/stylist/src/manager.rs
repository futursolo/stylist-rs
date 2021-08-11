use std::borrow::Cow;
use std::fmt;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::registry::StyleRegistry;
use crate::{Result, Style};

/// A trait to customise behaviour of [`Style`] and [`YieldStyle`](crate::YieldStyle).
pub trait StyleManager: fmt::Debug + Sync + Send {
    /// The default prefix used by the managed [`Style`] instances.
    fn prefix(&self) -> Cow<'static, str> {
        "stylist".into()
    }

    /// Returns an [`Arc<Mutex<StyleRegistry>>`] of [`StyleRegistry`].
    fn get_registry(&self) -> Arc<Mutex<StyleRegistry>>;

    /// Mount the [`Style`] in to the DOM tree.
    fn mount(&self, style: &Style) -> Result<()>;

    /// Unmount the [`Style`] from the DOM tree.
    fn unmount(&self, class_name: &str) -> Result<()>;
}

/// The default Style Manager.
#[derive(Debug, Default)]
pub(crate) struct DefaultManager;

impl StyleManager for DefaultManager {
    fn get_registry(&self) -> Arc<Mutex<StyleRegistry>> {
        static REGISTRY: Lazy<Arc<Mutex<StyleRegistry>>> = Lazy::new(Arc::default);
        REGISTRY.clone()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn mount(&self, _style: &Style) -> Result<()> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unmount(&self, _class_name: &str) -> Result<()> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    fn mount(&self, style: &Style) -> Result<()> {
        use crate::arch::{doc_head, document};
        use crate::Error;

        (|| {
            let document = document()?;
            let head = doc_head()?;

            let style_element = document.create_element("style")?;
            style_element.set_attribute("data-style", style.get_class_name())?;
            style_element.set_text_content(Some(style.get_style_str()));

            head.append_child(&style_element)?;
            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }

    #[cfg(target_arch = "wasm32")]
    fn unmount(&self, class_name: &str) -> Result<()> {
        use crate::arch::document;
        use crate::Error;

        (|| {
            let document = document()?;

            if let Some(m) =
                document.query_selector(&format!("style[data-style={}]", class_name))?
            {
                if let Some(parent) = m.parent_element() {
                    parent.remove_child(&m)?;
                }
            }

            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }
}
