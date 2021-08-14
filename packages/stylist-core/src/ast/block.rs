use std::fmt;

use super::{StyleAttribute, ToStyleStr};

/// A block is a set of css properties that apply to elements that
/// match the condition. The CSS standard calls these "Qualified rules".
///
/// E.g.:
/// ```css
/// .inner {
///     color: red;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    /// If set to [`None`], signals to substitute with the classname generated for the
    /// [`Sheet`](super::Sheet) in which this is conatined. Otherwise substitute the classname for
    /// each occuring '&', i.e. `None` is equivalent to `Some("&")`.
    pub condition: Option<String>,
    pub style_attributes: Vec<StyleAttribute>,
}

impl ToStyleStr for Block {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        if let Some(ref condition) = self.condition {
            if condition.contains('&') {
                let scoped_class = format!(".{}", class_name);
                writeln!(w, "{} {{", condition.replace("&", scoped_class.as_str()))?;
            } else {
                writeln!(w, ".{} {} {{", class_name, condition)?;
            }
        } else {
            writeln!(w, ".{} {{", class_name)?;
        }

        for attr in self.style_attributes.iter() {
            attr.write_style(w, class_name)?;
            writeln!(w)?;
        }

        write!(w, "}}")
    }
}
