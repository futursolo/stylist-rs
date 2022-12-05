use std::rc::Rc;

use crate::manager::{StyleId, StyleKey, StyleManager, WeakStyleManager};
use crate::Result;

#[derive(Debug)]
pub(crate) struct StyleContent {
    pub id: StyleId,
    pub key: Rc<StyleKey>,
    pub style_str: String,

    pub manager: WeakStyleManager,
}

impl StyleContent {
    pub fn id(&self) -> &StyleId {
        &self.id
    }

    pub fn get_style_str(&self) -> &str {
        &self.style_str
    }

    pub fn unmount(&self) -> Result<()> {
        StyleManager::unmount(self.id())
    }

    pub fn key(&self) -> &Rc<StyleKey> {
        &self.key
    }

    pub fn manager(&self) -> Option<StyleManager> {
        self.manager.upgrade()
    }

    pub fn unregister(&self) {
        if let Some(mgr) = self.manager() {
            mgr.unregister_style(self.key());
        }
    }
}

impl Drop for StyleContent {
    /// Unmounts the style from the HTML head web-sys style
    fn drop(&mut self) {
        let _result = self.unmount();
    }
}
