use std::borrow::Cow;

use crate::ast::SheetRef;
use crate::Result;

use super::IntoSheet;

impl IntoSheet for String {
    fn into_sheet(self) -> Result<SheetRef> {
        self.parse::<SheetRef>()
    }
}

impl IntoSheet for &str {
    fn into_sheet(self) -> Result<SheetRef> {
        self.parse::<SheetRef>()
    }
}

impl IntoSheet for Cow<'_, str> {
    fn into_sheet(self) -> Result<SheetRef> {
        self.parse::<SheetRef>()
    }
}
