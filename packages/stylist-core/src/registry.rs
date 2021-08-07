use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::{ast::Sheet, Style};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub(crate) struct StyleKey(pub Arc<Sheet>);

static REGISTRY: Lazy<Arc<Mutex<StyleRegistry>>> = Lazy::new(|| Arc::new(Mutex::default()));

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Debug, Default)]
pub(crate) struct StyleRegistry {
    styles: HashMap<Arc<StyleKey>, Style>,
}

impl StyleRegistry {
    pub fn get_ref() -> Arc<Mutex<StyleRegistry>> {
        REGISTRY.clone()
    }

    pub fn register(&mut self, style: Style) {
        if self.styles.insert(style.key(), style).is_some() {
            panic!("A Style with this StyleKey has already been created.");
        }
    }

    pub fn unregister(&mut self, key: &StyleKey) {
        self.styles.remove(key);
    }

    pub fn get(&self, key: &StyleKey) -> Option<Style> {
        self.styles.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::sample_scopes;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_duplicate_style() {
        init();

        let style_a = Style::try_from_scopes(sample_scopes()).unwrap();
        let style_b = Style::try_from_scopes(sample_scopes()).unwrap();

        {
            let reg = StyleRegistry::get_ref();
            let reg = reg.lock().unwrap();

            log::debug!("{:?}", reg);
        }

        assert_eq!(style_a.get_style_str(), style_b.get_style_str());
    }

    #[test]
    fn test_duplicate_style_different_prefix() {
        init();

        let style_a = Style::create_from_sheet("element-a", sample_scopes());
        let style_b = Style::create_from_sheet("element-b", sample_scopes());

        assert_eq!(style_a.get_style_str(), style_b.get_style_str());
    }

    #[test]
    fn test_unregister() {
        init();

        let style = Style::try_from_scopes(sample_scopes()).unwrap();

        {
            let reg = REGISTRY.clone();
            let reg = reg.lock().unwrap();

            assert!(reg.styles.get(&*style.key()).is_some());
        }

        style.unregister();

        {
            let reg = REGISTRY.clone();
            let reg = reg.lock().unwrap();

            assert!(reg.styles.get(&*style.key()).is_none());
        }
    }
}
