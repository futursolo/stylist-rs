use crate::ast::Sheet;
use crate::Result;

/// Turn a type into a stylesheet.
///
/// This trait works like [`TryInto<Sheet>`](std::convert::TryInto) but will always return [`Result<Sheet>`].
pub trait IntoSheet {
    /// Performs the conversion.
    fn into_sheet(self) -> Result<Sheet>;
}

impl<T: AsRef<str>> IntoSheet for T {
    fn into_sheet(self) -> Result<Sheet> {
        self.as_ref().parse()
    }
}

impl IntoSheet for Sheet {
    fn into_sheet(self) -> Result<Sheet> {
        Ok(self)
    }
}
