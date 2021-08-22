use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use super::{ScopeContent, ToStyleStr};
use crate::parser::Parser;
use crate::Result;

/// The top node of a style string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sheet(Arc<Cow<'static, [ScopeContent]>>);

impl FromStr for Sheet {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let m = Parser::parse(s)?;

        Ok(m)
    }
}

impl Deref for Sheet {
    type Target = [ScopeContent];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sheet {
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
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> Result<()> {
        for scope in self.0.iter() {
            scope.write_style(w, class_name)?;
            writeln!(w)?;
        }

        Ok(())
    }
}
