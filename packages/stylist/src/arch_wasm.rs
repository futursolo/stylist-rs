use crate::{Error, Result};
pub use wasm_bindgen::JsValue;
use web_sys::{Document, HtmlHeadElement, Window};

pub(crate) fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| Error::Web(None))
}

pub(crate) fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| Error::Web(None))
}

pub(crate) fn doc_head() -> Result<HtmlHeadElement> {
    document()?.head().ok_or_else(|| Error::Web(None))
}
