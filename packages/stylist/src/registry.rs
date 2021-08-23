use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::SheetRef;
use crate::style::StyleContent;

/// A [`StyleKey`].
///
/// Every Style that has the same [`StyleKey`] will be considered as the same style in the
/// registry.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub(crate) struct StyleKey {
    pub is_global: bool,
    pub prefix: Cow<'static, str>,
    pub ast: SheetRef,
}

/// The style registry is a registry that keeps an instance of all styles for current manager.
#[derive(Debug, Default)]
pub(crate) struct StyleRegistry {
    styles: HashMap<Rc<StyleKey>, Rc<StyleContent>>,
}

impl StyleRegistry {
    pub(crate) fn register(&mut self, content: Rc<StyleContent>) {
        let key = content.key().clone();
        if self.styles.insert(key, content).is_some() {
            panic!("A Style with this StyleKey has already been created.");
        }
    }

    pub(crate) fn unregister(&mut self, key: Rc<StyleKey>) {
        self.styles.remove(&key);
    }

    pub(crate) fn get(&self, key: &StyleKey) -> Option<Rc<StyleContent>> {
        self.styles.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
    use crate::manager::StyleManager;
    use crate::*;

    fn sample_scopes() -> SheetRef {
        "color: red;".parse().expect("Failed to Parse style.")
    }

    fn get_registry() -> Rc<RefCell<StyleRegistry>> {
        StyleManager::default().get_registry()
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
            let reg = get_registry();
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
            let reg = get_registry();
            let reg = reg.borrow_mut();

            assert!(reg.styles.get(&*style.key()).is_some());
        }

        style.unregister();

        {
            let reg = get_registry();
            let reg = reg.borrow_mut();

            assert!(reg.styles.get(&*style.key()).is_none());
        }
    }
}
