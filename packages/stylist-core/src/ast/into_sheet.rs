use std::borrow::Cow;

use crate::ast::Sheet;
use crate::Result;

/// Turn a type into a stylesheet.
///
/// This trait works like [`TryInto<Sheet>`](std::convert::TryInto) but will always return [`Result<Sheet>`].
pub trait IntoSheet<'a> {
    /// Performs the conversion.
    fn into_sheet(self) -> Result<Cow<'a, Sheet>>;
}

impl<'a, T: AsRef<str>> IntoSheet<'a> for T {
    fn into_sheet(self) -> Result<Cow<'a, Sheet>> {
        self.as_ref().parse::<Sheet>().map(Cow::Owned)
    }
}

impl<'a> IntoSheet<'a> for Sheet {
    fn into_sheet(self) -> Result<Cow<'a, Sheet>> {
        Ok(Cow::Owned(self))
    }
}

impl<'a> IntoSheet<'a> for &'a Sheet {
    fn into_sheet(self) -> Result<Cow<'a, Sheet>> {
        Ok(Cow::Borrowed(self))
    }
}
