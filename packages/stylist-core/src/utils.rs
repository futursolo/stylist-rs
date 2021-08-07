use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::{Document, HtmlHeadElement, Window};

static RNG: Lazy<Arc<Mutex<SmallRng>>> =
    Lazy::new(|| Arc::new(Mutex::new(SmallRng::from_entropy())));

pub(crate) fn get_rand_str() -> String {
    let mut rng = RNG.lock().expect("Failed to lock Rng.");

    (&mut *rng)
        .sample_iter(Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or(JsValue::UNDEFINED)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn document() -> Result<Document, JsValue> {
    window()?.document().ok_or(JsValue::UNDEFINED)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn doc_head() -> Result<HtmlHeadElement, JsValue> {
    document()?.head().ok_or(JsValue::UNDEFINED)
}
