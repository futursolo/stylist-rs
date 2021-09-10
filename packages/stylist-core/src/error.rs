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
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultDisplay<T> {
    /// Returns the contained Ok value, consuming the self value, panic when `Result` is `Err`.
    fn unwrap_display(self) -> T;
    /// Returns the contained Ok value, consuming the self value, panic with message when `Result` is `Err`.
    fn expect_display(self, msg: &str) -> T;
}

impl<T> ResultDisplay<T> for Result<T> {
    fn unwrap_display(self) -> T {
        match self {
            Ok(m) => m,
            Err(e) => panic!("{}", e),
        }
    }

    fn expect_display(self, msg: &str) -> T {
        match self {
            Ok(m) => m,
            Err(e) => panic!("{}: {}", msg, e),
        }
    }
}
