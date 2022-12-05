use core::fmt;
use std::borrow::Cow;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::ast::Sheet;
use crate::utils::get_entropy;

/// A [`StyleKey`].
///
/// Every Style that has the same [`StyleKey`] will be considered as the same style in the
/// registry.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub(crate) struct StyleKey {
    pub is_global: bool,
    pub prefix: Cow<'static, str>,
    pub ast: Sheet,
}

/// The Unique Identifier of a Style.
///
/// This is primarily used by [`StyleManager`] to track the mounted instance of [`Style`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StyleId(String);

impl Deref for StyleId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl fmt::Display for StyleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StyleId {
    pub(crate) fn new_scoped(prefix: &str) -> StyleId {
        StyleId(format!("{}-{}", prefix, get_entropy()))
    }

    pub(crate) fn new_global(prefix: &str) -> StyleId {
        StyleId(format!("{}-global-{}", prefix, get_entropy()))
    }
}
