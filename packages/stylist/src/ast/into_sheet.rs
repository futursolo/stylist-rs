#[cfg(feature = "parser")]
use std::borrow::Cow;

use crate::ast::SheetRef;
use crate::Result;

/// Turn a type into a stylesheet.
pub trait IntoSheet {
    /// Performs the conversion.
    fn into_sheet(self) -> Result<SheetRef>;
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl IntoSheet for String {
    fn into_sheet(self) -> Result<SheetRef> {
        self.parse::<SheetRef>()
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl IntoSheet for &str {
    fn into_sheet(self) -> Result<SheetRef> {
        self.parse::<SheetRef>()
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl IntoSheet for Cow<'_, str> {
    fn into_sheet(self) -> Result<SheetRef> {
        self.parse::<SheetRef>()
    }
}

impl IntoSheet for SheetRef {
    fn into_sheet(self) -> Result<SheetRef> {
        Ok(self)
    }
}
