use std::ops::Range;

use super::rtokens::RTokenStream;

/// A struct that provides location information
#[derive(Debug, Clone)]
pub enum Location {
    Literal {
        /// The token of the string literal.
        /// not available if generated from runtime string.
        token: Option<RTokenStream>,
        range: Range<usize>,
    },
    Span(RTokenStream),
}
