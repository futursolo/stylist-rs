use std::borrow::Cow;
use std::fmt;

use crate::Result;

// #[derive(Debug)]
// pub enum StyleKind {
//     Style,
//     Keyframes,
// }
//

#[derive(Debug, Clone, PartialEq)]
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
        // @ rules first, then selectors.
        // Equivalent to the following line, but would result in a smaller bundle
        // sorted_parents.sort_by_cached_key(|m| !m.starts_with('@'));
        let (mut rules, mut selectors) = self.parent_conditions.clone().into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut rules, mut selectors), item| {
                if item.starts_with('@') {
                    rules.push(item);
                } else {
                    selectors.push(item);
                }

                (rules, selectors)
            },
        );

        rules.append(&mut selectors);
        rules
    }

    pub fn write_starting_clause<W: fmt::Write>(&mut self, w: &mut W) -> Result<()> {
        if self.state == ContextState::Closed {
            for (index, cond) in self.parent_conditions().iter().enumerate() {
                for _i in 0..index {
                    write!(w, "    ")?;
                }
                writeln!(w, "{} {{", cond)?;
            }

            self.state = ContextState::Open;
        }

        Ok(())
    }

    pub fn write_finishing_clause<W: fmt::Write>(&mut self, w: &mut W) -> Result<()> {
        if self.state == ContextState::Open {
            for i in (0..self.parent_conditions.len()).rev() {
                for _i in 0..i {
                    write!(w, "    ")?;
                }
                writeln!(w, "}}")?;
            }

            self.state = ContextState::Closed;
        }

        Ok(())
    }

    pub fn write_padding<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
        for _ in 0..self.parent_conditions.len() {
            write!(w, "    ")?;
        }

        Ok(())
    }
}
