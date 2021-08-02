use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

#[doc(hidden)]
pub mod ast;
use crate::utils::get_rand_str;

use super::parser::Parser;
use ast::Scope;
#[cfg(target_arch = "wasm32")]
use ast::ToCss;
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

static STYLE_REGISTRY: Lazy<Arc<Mutex<StyleRegistry>>> = Lazy::new(|| Arc::new(Mutex::default()));

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Debug, Default, Clone)]
struct StyleRegistry {
    styles: HashMap<String, Style>,
}

#[derive(Debug, Clone)]
pub struct Style {
    /// The designated class name of this style
    class_name: String,

    /// The abstract syntax tree of the css
    ast: Arc<Vec<Scope>>,
}

impl Style {
    /// Creates a new style
    pub fn new<S: Into<String>>(css: S) -> Result<Self, String> {
        Self::create("stylist", css)
    }

    /// Returns the class name for current style
    pub fn get_class_name(&self) -> &str {
        &self.class_name
    }

    /// Creates a new style with custom class prefix
    pub fn create<I1: Into<String>, I2: Into<String>>(
        class_name: I1,
        css: I2,
    ) -> Result<Style, String> {
        let (class_name, css) = (class_name.into(), css.into());

        let ast = Parser::parse(css)?;
        let new_style = Self {
            class_name: format!("{}-{}", class_name, get_rand_str()),
            ast: Arc::new(ast),
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
            .insert(new_style.class_name.clone(), new_style.clone());

        Ok(new_style)
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

    /// Takes all Scopes and lets them translate themselves into CSS.
    fn generate_css(&self) -> String {
        self.ast
            .iter()
            .map(|scope| scope.to_css(self.class_name.clone()))
            .fold(String::new(), |mut acc, css_part| {
                acc.push('\n');
                acc.push_str(&css_part);
                acc
            })
    }

    /// Generates the `<style/>` tag web-sys style for inserting into the head of the
    /// HTML document.
    fn generate_element(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let style_element = document.create_element("style").unwrap();
        style_element
            .set_attribute("data-style", self.class_name.as_str())
            .ok();
        style_element.set_text_content(Some(self.generate_css().as_str()));
        Ok(style_element)
    }
}

impl ToString for Style {
    /// Just returns the classname
    fn to_string(&self) -> String {
        self.class_name.clone()
    }
}
