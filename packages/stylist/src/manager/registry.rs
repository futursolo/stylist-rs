use std::collections::HashMap;
use std::rc::Rc;

use super::{StyleContent, StyleKey};

/// The style registry is a registry that keeps an instance of all styles for current manager.
#[derive(Debug, Default)]
pub(super) struct StyleRegistry {
    pub(crate) styles: HashMap<Rc<StyleKey>, Rc<StyleContent>>,
}

impl StyleRegistry {
    pub fn register(&mut self, content: Rc<StyleContent>) {
        if self.styles.insert(content.key().clone(), content).is_some() {
            panic!("A Style with this StyleKey has already been created.");
        }
    }

    pub fn unregister(&mut self, key: &StyleKey) {
        self.styles.remove(key);
    }

    pub fn get(&self, key: &StyleKey) -> Option<Rc<StyleContent>> {
        self.styles.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Sheet;
    use crate::manager::StyleManager;
    use crate::*;

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
            let mgr = StyleManager::default();
            let reg = mgr.get_registry();
            let reg = reg.borrow_mut();

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
            let mgr = StyleManager::default();
            let reg = mgr.get_registry();
            let reg = reg.borrow_mut();

            assert!(reg.styles.contains_key(style.key()));
        }

        style.unregister();

        {
            let mgr = StyleManager::default();
            let reg = mgr.get_registry();
            let reg = reg.borrow_mut();

            assert!(reg.styles.contains_key(style.key()));
        }
    }
}
