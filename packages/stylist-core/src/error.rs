use std::fmt;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// Failed to parse CSS.
    #[cfg_attr(documenting, doc(cfg(feature = "parser")))]
    #[cfg(feature = "parser")]
    #[error("Failed to Parse CSS, due to:\n{}", .reason)]
    Parse {
        reason: String,
        #[source]
        source: Option<nom::error::VerboseError<String>>,
    },

    /// Failed to interact with Web API.
    ///
    /// This is usually raised when the style element failed to mount.
    #[error("Failed to Interact with Web API. Are you running in Browser?")]
    Web(Option<wasm_bindgen::JsValue>),

    /// Format error when writing Styles.
    #[error("Failed to write style!")]
    Fmt(#[from] fmt::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
