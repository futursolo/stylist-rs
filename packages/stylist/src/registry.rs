use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::ast::Sheet;
use crate::Style;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub(crate) struct StyleKey<'a> {
    pub prefix: Cow<'static, str>,
    pub ast: Cow<'a, Sheet>,
}

static REGISTRY: Lazy<Arc<Mutex<StyleRegistry>>> = Lazy::new(|| Arc::new(Mutex::default()));

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Debug, Default)]
pub(crate) struct StyleRegistry {
    styles: HashMap<Arc<StyleKey<'static>>, Style>,
}

impl StyleRegistry {
    pub fn get_ref() -> Arc<Mutex<StyleRegistry>> {
        REGISTRY.clone()
    }

    pub fn register(&mut self, style: Style) {
        let key = style.key().clone();
        if self.styles.insert(key, style).is_some() {
            panic!("A Style with this StyleKey has already been created.");
        }
    }

    pub fn unregister(&mut self, key: Arc<StyleKey<'static>>) {
        self.styles.remove(&key);
    }

    pub fn get(&self, key: &StyleKey<'_>) -> Option<Style> {
        self.styles.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylist_core::ast::*;

    fn sample_scopes() -> Sheet {
        "color: red;".parse().expect("Failed to Parse style.")
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_duplicate_style() {
        init();

        let style_a = Style::new(sample_scopes()).expect("Failed to create Style.");
        let style_b = Style::new(sample_scopes()).expect("Failed to create Style.");

        {
            let reg = StyleRegistry::get_ref();
            let reg = reg.lock().unwrap();

            log::debug!("{:#?}", reg);
        }

        assert_eq!(style_a.get_style_str(), style_b.get_style_str());
    }

    #[test]
    fn test_duplicate_style_different_prefix() {
        init();

        let style_a = Style::create("element-a", sample_scopes()).expect("Failed to create Style.");
        let style_b = Style::create("element-b", sample_scopes()).expect("Failed to create Style.");

        assert_ne!(style_a.get_class_name(), style_b.get_class_name());
    }

    #[test]
    fn test_unregister() {
        init();

        let style = Style::create(
            "super-secret-prefix-for-unregister-that-never-gets-collided",
            sample_scopes(),
        )
        .expect("Failed to create Style.");

        {
            let reg = StyleRegistry::get_ref();
            let reg = reg.lock().unwrap();

            assert!(reg.styles.get(&*style.key()).is_some());
        }

        style.unregister();

        {
            let reg = StyleRegistry::get_ref();
            let reg = reg.lock().unwrap();

            assert!(reg.styles.get(&*style.key()).is_none());
        }
    }
}
