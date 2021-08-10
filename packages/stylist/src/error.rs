use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Failed to parse CSS.
    #[error("Failed to Parse CSS, due to: {}", .0)]
    Parse(String),

    /// Failed to interact with Web API.
    ///
    /// This is usually raised when the style element failed to mount.
    #[error("Failed to Interact with Web API. Are you running in Browser?")]
    Web(Option<wasm_bindgen::JsValue>),
}

pub type Result<T> = std::result::Result<T, Error>;
