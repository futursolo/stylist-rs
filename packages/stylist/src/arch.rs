use crate::{Error, Result};
pub use wasm_bindgen::JsValue;
use web_sys::Window;
#[cfg(all(target_arch = "wasm32", not(feature = "wasi")))]
use web_sys::{Document, HtmlHeadElement};

pub(crate) fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::Web(None))
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasi")))]
pub(crate) fn document() -> Result<Document> {
    window()?.document().ok_or(Error::Web(None))
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasi")))]
pub(crate) fn doc_head() -> Result<HtmlHeadElement> {
    document()?.head().ok_or(Error::Web(None))
}
