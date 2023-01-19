use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{ScopeContent, StyleContext, ToStyleStr};

/// The top node of a stylesheet.
// Once a sheet is constructed, it becomes immutable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sheet(Arc<Cow<'static, [ScopeContent]>>);

impl Serialize for Sheet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_ref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Sheet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
            .map(Arc::new)
            .map(Self)
    }
}

impl Deref for Sheet {
    type Target = [ScopeContent];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sheet {
    /// Creates an empty stylesheet.
    pub fn new() -> Self {
        Self(Arc::new(Cow::Borrowed(&[])))
    }
}

impl From<Vec<ScopeContent>> for Sheet {
    fn from(v: Vec<ScopeContent>) -> Self {
        Self(Arc::new(v.into()))
    }
}

impl From<&'static [ScopeContent]> for Sheet {
    fn from(v: &'static [ScopeContent]) -> Self {
        Self(Arc::new(v.into()))
    }
}

impl From<Cow<'static, [ScopeContent]>> for Sheet {
    fn from(v: Cow<'static, [ScopeContent]>) -> Self {
        Self(Arc::new(v))
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

impl ToStyleStr for Sheet {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        for scope in self.0.iter() {
            scope.write_style(w, ctx);
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::{Arc, Mutex};

    use super::*;

    static CACHED_SHEETS: Lazy<Arc<Mutex<HashMap<String, Sheet>>>> = Lazy::new(Arc::default);

    impl FromStr for Sheet {
        type Err = crate::Error;

        fn from_str(s: &str) -> crate::Result<Self> {
            use crate::parser::Parser;

            let cached_sheets = CACHED_SHEETS.clone();
            let mut cached_sheets = cached_sheets.lock().unwrap();

            if let Some(m) = cached_sheets.get(s) {
                return Ok(m.clone());
            }

            let m: Sheet = Parser::parse(s)?;

            cached_sheets.insert(s.to_string(), m.clone());

            Ok(m)
        }
    }
}
