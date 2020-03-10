// Copyright © 2020 Lukas Wagner

extern crate rand;
extern crate web_sys;

pub mod ast;

use super::parser::Parser;
use ast::{Scope, ToCss};
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use web_sys::Element;

use wasm_bindgen::prelude::*;

lazy_static! {
    static ref STYLE_REGISTRY: Arc<Mutex<StyleRegistry>> = Arc::new(Mutex::default());
}

#[derive(Debug, Clone)]
struct StyleRegistry {
    styles: HashMap<String, Style>,
}

impl Default for StyleRegistry {
    fn default() -> Self {
        Self {
            styles: HashMap::new(),
        }
    }
}

unsafe impl Send for StyleRegistry {}
unsafe impl Sync for StyleRegistry {}

#[derive(Debug, Clone)]
pub struct Style {
    /// The designated class name of this style
    pub class_name: String,
    /// The abstract syntax tree of the css
    ast: Option<Vec<Scope>>,
    /// Style node the data in this struct is turned into.
    node: Option<Element>,
}

impl Style {
    // Creates a new style and, stores it into the registry and returns the
    pub fn create(class_name: String, css: String) -> Style {
        let small_rng = SmallRng::from_entropy();
        let mut new_style = Self {
            class_name: format!(
                "{}-{}",
                class_name,
                small_rng
                    .sample_iter(Alphanumeric)
                    .take(30)
                    .collect::<String>()
            ),
            // TODO log out an error
            ast: Parser::parse(css).ok(),
            node: None,
        };
        new_style = new_style.mount();
        let style_registry_mutex = Arc::clone(&STYLE_REGISTRY);
        let mut style_registry = match style_registry_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        (*style_registry)
            .styles
            .insert(new_style.class_name.clone(), new_style.clone());
        new_style
    }

    fn mount(&mut self) -> Self {
        let mut style = self.unmount();
        style.node = self.generate_element().ok();
        if let Some(node) = style.node {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let head = document.head().expect("should have a head in document");
            head.append_child(&node).ok();
        }
        self.clone()
    }

    fn unmount(&mut self) -> Self {
        if let Some(node) = &self.node {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let head = document.head().expect("should have a head in document");
            head.remove_child(node).ok();
        }
        self.clone()
    }

    fn generate_css(&self) -> String {
        match &self.ast {
            Some(ast) => ast
                .clone()
                .into_iter()
                .map(|scope| scope.to_css(self.class_name.clone()))
                .fold(String::new(), |acc, css_part| {
                    format!("{}\n{}", acc, css_part)
                }),
            None => String::new(),
        }
    }

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
    fn to_string(&self) -> String {
        self.class_name.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
