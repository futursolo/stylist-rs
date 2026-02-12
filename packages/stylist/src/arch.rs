use crate::{Error, Result};
use web_sys::Window;
#[cfg(target_arch = "wasm32")]
use web_sys::{Document, HtmlHeadElement};

pub(crate) fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::Web(None))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn document() -> Result<Document> {
    window()?.document().ok_or(Error::Web(None))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn doc_head() -> Result<HtmlHeadElement> {
    document()?.head().ok_or(Error::Web(None))
}
