use std::ops::Deref;
use std::sync::Arc;

use crate::ast::Sheet;

/// An Arc'ed Version of [`Sheet`](super::Sheet).
///
/// This struct also implements [`FromStr`](std::str::FromStr) if this crate is compiled with feature `parser`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SheetRef(Arc<Sheet>);

impl From<Sheet> for SheetRef {
    fn from(other: Sheet) -> Self {
        Self(Arc::new(other))
    }
}

impl Deref for SheetRef {
    type Target = Sheet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::Mutex;

    use super::*;

    static CACHED_SHEETS: Lazy<Arc<Mutex<HashMap<String, SheetRef>>>> = Lazy::new(Arc::default);

    impl FromStr for SheetRef {
        type Err = crate::Error;

        fn from_str(s: &str) -> crate::Result<Self> {
            use stylist_parser::Parser;

            let cached_sheets = CACHED_SHEETS.clone();
            let mut cached_sheets = cached_sheets.lock().unwrap();

            if let Some(m) = cached_sheets.get(s) {
                return Ok(m.clone());
            }

            let m: SheetRef = Parser::parse(s)?.into();

            cached_sheets.insert(s.to_string(), m.clone());

            Ok(m)
        }
    }
}
