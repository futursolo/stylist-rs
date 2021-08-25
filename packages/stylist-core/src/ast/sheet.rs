use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;

use super::{ScopeContent, ToStyleStr};

use crate::Result;

/// The top node of a style string.
// Once a sheet is constructed, it becomes immutable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sheet(Cow<'static, [ScopeContent]>);

impl Deref for Sheet {
    type Target = [ScopeContent];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sheet {
    pub fn new() -> Self {
        Self(Cow::Borrowed(&[]))
    }
}

impl From<Vec<ScopeContent>> for Sheet {
    fn from(v: Vec<ScopeContent>) -> Self {
        Self(v.into())
    }
}

impl From<&'static [ScopeContent]> for Sheet {
    fn from(v: &'static [ScopeContent]) -> Self {
        Self(v.into())
    }
}

impl From<Cow<'static, [ScopeContent]>> for Sheet {
    fn from(v: Cow<'static, [ScopeContent]>) -> Self {
        Self(v)
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
