#[cfg(feature = "proc_macro_support")]
mod loc_impl {
    use std::ops::Range;

    use proc_macro2 as r;

    /// A struct that provides location information.
    #[derive(Debug, Clone)]
    pub enum Location {
        /// The location of a string literal.
        Literal {
            /// The token of the string literal.
            /// not available if generated from runtime string.
            token: Option<r::TokenStream>,
            range: Range<usize>,
        },
        /// The location of a [`proc_macro2::TokenStream`] in the form of [`Span`](proc_macro2::Span).
        Span(r::Span),
        /// The location of a [`proc_macro2::TokenStream`].
        TokenStream(r::TokenStream),
    }
}

#[cfg(not(feature = "proc_macro_support"))]
mod loc_impl {
    use std::ops::Range;

    /// A struct that provides location information.
    #[derive(Debug, Clone)]
    pub enum Location {
        /// The location of a string literal.
        Literal { range: Range<usize> },
    }
}

pub use loc_impl::Location;
