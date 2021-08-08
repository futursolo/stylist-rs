pub use wasm_bindgen::JsValue;
use web_sys::{Document, HtmlHeadElement, Window};

pub(crate) fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or(JsValue::UNDEFINED)
}

pub(crate) fn document() -> Result<Document, JsValue> {
    window()?.document().ok_or(JsValue::UNDEFINED)
}

pub(crate) fn doc_head() -> Result<HtmlHeadElement, JsValue> {
    document()?.head().ok_or(JsValue::UNDEFINED)
}
