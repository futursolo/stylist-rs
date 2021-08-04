use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::Style;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub(crate) struct StyleKey(pub String, pub Cow<'static, str>);

impl PartialEq<(&str, &str)> for StyleKey {
    fn eq(&self, other: &(&str, &str)) -> bool {
        &(self.0.as_str(), &*self.1) == other
    }
}

static REGISTRY: Lazy<Arc<Mutex<StyleRegistry>>> = Lazy::new(|| Arc::new(Mutex::default()));

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Debug, Default)]
pub(crate) struct StyleRegistry {
    styles: HashMap<Arc<StyleKey>, Style>,
}

impl StyleRegistry {
    pub fn register(style: Style) {
        let reg = REGISTRY.clone();
        let mut reg = reg.lock().unwrap();

        if reg.styles.insert(style.key(), style).is_some() {
            panic!("A Style with this StyleKey has already been created.");
        }
    }

    pub fn unregister(key: &StyleKey) {
        let reg = REGISTRY.clone();
        let mut reg = reg.lock().unwrap();

        reg.styles.remove(key);
    }

    pub fn get(key: &StyleKey) -> Option<Style> {
        let reg = REGISTRY.clone();
        let reg = reg.lock().unwrap();

        reg.styles.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_duplicate_style() -> Result<()> {
        let style_a = Style::new(r#"color: red;"#)?;
        let style_b = Style::new(r#"color: red;"#)?;

        assert_eq!(style_a.get_style_str(), style_b.get_style_str());
        Ok(())
    }

    #[test]
    fn test_duplicate_style_different_prefix() -> Result<()> {
        let style_a = Style::create("element-a", r#"color: red;"#)?;
        let style_b = Style::create("element-b", r#"color: red;"#)?;

        assert_ne!(style_a.get_style_str(), style_b.get_style_str());
        Ok(())
    }

    #[test]
    fn test_unregister() -> Result<()> {
        let style = Style::new(r#"color: red;"#)?;

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

        Ok(())
    }
}
