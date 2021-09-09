use std::borrow::Cow;
use std::fmt;

use crate::Result;

// #[derive(Debug)]
// pub enum StyleKind {
//     Style,
//     Keyframes,
// }
//

#[derive(Debug, Clone, PartialEq, Eq)]
enum ContextState {
    // Either a finishing clause has been printed, or the starting block is not printed.
    Closed,
    // A start clause has been printed, but a finishing clause is not printed.
    Open,
}

#[derive(Debug, Clone)]
pub struct StyleContext<'a> {
    // pub kind: StyleKind,
    pub class_name: Option<&'a str>,
    pub parent_conditions: Vec<Cow<'a, str>>,
    state: ContextState,
}

impl<'a> StyleContext<'a> {
    pub fn new(class_name: Option<&'a str>) -> Self {
        Self {
            class_name,
            parent_conditions: Vec::new(),
            state: ContextState::Closed,
        }
    }

    pub fn with_condition<S: Into<Cow<'a, str>>>(&self, condition: S) -> Self {
        let mut self_ = self.clone();

        self_.parent_conditions.push(condition.into());

        self_
    }

    pub fn to_block_context(&'a self) -> Self {
        // No selectors
        if self
            .parent_conditions()
            .last()
            .map(|m| m.starts_with('@'))
            .unwrap_or(true)
        {
            self.with_condition(
                self.class_name
                    .map(|m| Cow::from(format!(".{}", m)))
                    .unwrap_or_else(|| "html".into()),
            )
        } else {
            self.clone()
        }
    }

    pub fn parent_conditions(&self) -> Vec<Cow<'a, str>> {
        let mut sorted_parents = self.parent_conditions.clone();

        // @ rules first, then selectors.
        sorted_parents.sort_by_cached_key(|m| !m.starts_with('@'));

        sorted_parents
    }

    pub fn write_starting_clause<W: fmt::Write>(&mut self, w: &mut W) -> Result<()> {
        if self.state == ContextState::Open {
            return Ok(());
        }

        for (index, cond) in self.parent_conditions().iter().enumerate() {
            for _i in 0..index {
                write!(w, "    ")?;
            }
            writeln!(w, "{} {{", cond)?;
        }

        self.state = ContextState::Open;

        Ok(())
    }

    pub fn write_finishing_clause<W: fmt::Write>(&mut self, w: &mut W) -> Result<()> {
        if self.state == ContextState::Closed {
            return Ok(());
        }

        for i in (0..self.parent_conditions.len()).rev() {
            for _i in 0..i {
                write!(w, "    ")?;
            }
            writeln!(w, "}}")?;
        }

        self.state = ContextState::Closed;

        Ok(())
    }

    pub fn write_padding<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
        for _ in 0..self.parent_conditions.len() {
            write!(w, "    ")?;
        }

        Ok(())
    }
}
