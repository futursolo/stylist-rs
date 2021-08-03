use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to Parse CSS, due to: {}", .0)]
    Parse(String),

    #[error("Failed to Interact with Web API. Are you running in Browser?")]
    Web(Option<wasm_bindgen::JsValue>),
}

pub type Result<T> = std::result::Result<T, Error>;
