use once_cell::sync::OnceCell;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

use crate::ast::{Scope, ToCss};
use crate::parser::Parser;
use crate::utils::get_rand_str;

static STYLE_REGISTRY: Lazy<Arc<Mutex<StyleRegistry>>> = Lazy::new(|| Arc::new(Mutex::default()));

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Debug, Default)]
struct StyleRegistry {
    styles: HashMap<String, Style>,
}

#[derive(Debug)]
struct StyleContent {
    /// The designated class name of this style
    class_name: String,

    /// The abstract syntax tree of the css
    ast: Arc<Vec<Scope>>,

    style_str: OnceCell<String>,
}

#[derive(Debug, Clone)]
pub struct Style {
    inner: Arc<StyleContent>,
}

impl Style {
    /// Creates a new style
    pub fn new<S: Into<Cow<'static, str>>>(css: S) -> Result<Self, String> {
        Self::create("stylist", css)
    }

    /// Returns the class name for current style
    pub fn get_class_name(&self) -> &str {
        &self.inner.class_name
    }

    /// Creates a new style with custom class prefix
    pub fn create<I1: AsRef<str>, I2: Into<Cow<'static, str>>>(
        class_prefix: I1,
        css: I2,
    ) -> Result<Style, String> {
        let (class_prefix, css) = (class_prefix.as_ref(), css.into());

        let ast = Parser::parse(&*css)?;
        let new_style = Self {
            inner: Arc::new(StyleContent {
                class_name: format!("{}-{}", class_prefix, get_rand_str()),
                ast: Arc::new(ast),
                style_str: OnceCell::new(),
            }),
        };

        #[cfg(target_arch = "wasm32")]
        new_style.mount();

        let style_registry_mutex = Arc::clone(&STYLE_REGISTRY);
        let mut style_registry = match style_registry_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        (*style_registry)
            .styles
            .insert(new_style.get_class_name().to_string(), new_style.clone());

        Ok(new_style)
    }

    /// Get the parsed and generated style in `&str`.
    pub fn get_style_str(&self) -> &str {
        self.inner.style_str.get_or_init(|| {
            self.inner
                .ast
                .iter()
                .map(|scope| scope.to_css(self.get_class_name()))
                .fold(String::new(), |mut acc, css_part| {
                    acc.push('\n');
                    acc.push_str(&css_part);
                    acc
                })
        })
    }
}

#[cfg(target_arch = "wasm32")]
impl Style {
    /// Mounts the styles to the document head web-sys style
    fn mount(&self) {
        if let Ok(node) = self.generate_element() {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let head = document.head().expect("should have a head in document");
            head.append_child(&node).ok();
        }
    }

    // Unmounts the style from the HTML head web-sys style
    // fn unmount(&mut self) -> Self {
    //    let window = web_sys::window().expect("no global `window` exists");
    //    let document = window.document().expect("should have a document on window");
    //
    //   if let Some(m) = document
    //       .query_selector(&format!("style[data-style={}]", self.class_name))
    //       .ok()
    //       .and_then(|m| m)
    //   {
    //       if let Some(parent) = m.parent_element() {
    //           let _result = parent.remove_child(&m);
    //       }
    //   }
    //   self.clone()
    //}

    /// Generates the `<style/>` tag web-sys style for inserting into the head of the
    /// HTML document.
    fn generate_element(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let style_element = document.create_element("style").unwrap();
        style_element
            .set_attribute("data-style", self.get_class_name())
            .ok();
        style_element.set_text_content(Some(self.get_style_str()));
        Ok(style_element)
    }
}

impl ToString for Style {
    /// Just returns the classname
    fn to_string(&self) -> String {
        self.get_class_name().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        Style::new("background-color: black;").expect("Failed to create Style.");
    }

    #[test]
    fn test_complex() {
        let style = Style::new(
            r#"
                background-color: black;

                .with-class {
                    color: red;
                }

                @media screen and (max-width: 600px) {
                    color: yellow;
                }
            "#,
        )
        .expect("Failed to create Style.");

        assert_eq!(
            style.get_style_str(),
            format!(
                r#"
.{style_name} {{
background-color: black;
}}
.{style_name} .with-class {{
color: red;
}}

@media screen and (max-width: 600px) {{
.{style_name} {{
color: yellow;
}}
}}"#,
                style_name = style.get_class_name()
            )
        )
    }
}
