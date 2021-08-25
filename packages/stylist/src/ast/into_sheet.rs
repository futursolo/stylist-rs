use crate::ast::SheetRef;
use crate::Result;

/// Turn a type into a stylesheet.
pub trait IntoSheet {
    /// Performs the conversion.
    fn into_sheet(self) -> Result<SheetRef>;
}

impl IntoSheet for SheetRef {
    fn into_sheet(self) -> Result<SheetRef> {
        Ok(self)
    }
}
