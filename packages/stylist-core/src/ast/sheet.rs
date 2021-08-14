use std::fmt;
use std::str::FromStr;

use crate::parser::Parser;

use super::{ScopeContent, ToStyleStr};

/// The top node of a style string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sheet(pub Vec<ScopeContent>);

impl FromStr for Sheet {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Parser::parse(s)
    }
}

impl Sheet {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

impl ToStyleStr for Sheet {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        for scope in self.0.iter() {
            scope.write_style(w, class_name)?;
            writeln!(w)?;
        }

        Ok(())
    }
}
