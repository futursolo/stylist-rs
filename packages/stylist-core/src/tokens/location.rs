use std::ops::Range;

use super::rtokens::{RSpan, RTokenStream};

/// A struct that provides location information.
#[derive(Debug, Clone)]
pub enum Location {
    /// The location of a string literal.
    Literal {
        /// The token of the string literal.
        /// not available if generated from runtime string.
        token: Option<RTokenStream>,
        range: Range<usize>,
    },
    /// The location of a [`proc_macro2::TokenStream`] in the form of [`Span`](proc_macro2::Span).
    Span(RSpan),
    /// The location of a [`proc_macro2::TokenStream`].
    TokenStream(RTokenStream),
}
