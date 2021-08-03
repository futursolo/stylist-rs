use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use web_sys::{Document, HtmlHeadElement, Window};

use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

#[cfg(target_arch = "wasm32")]
use crate::{Error, Result};

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
