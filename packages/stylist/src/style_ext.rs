use std::borrow::Borrow;
use std::result::Result;
use stylist_core::ast::Sheet;

pub type Style = stylist_core::Style;

/// Trait for types that can be parsed into css [`Sheets`].
///
/// [`Sheets`]: `stylist_core::ast::Sheet`
pub trait TryParseCss {
    /// Error type encountered during parsing
    type Error;
    /// Try to convert a self to a css [`Sheet`].
    fn try_parse(self) -> Result<Sheet, Self::Error>;
}

impl<'a> TryParseCss for &'a str {
    type Error = crate::error::Error;

    fn try_parse(self) -> Result<Sheet, Self::Error> {
        crate::parser::Parser::parse(self)
    }
}

pub trait StyleExt: Sized {
    /// Creates a new style with a custom class prefix from some parsable css.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylist::{Style, StyleExt};
    ///
    /// let style = Style::create("my-component", "background-color: red;")?;
    /// # use stylist::TryParseCss;
    /// # Ok::<(), <&str as TryParseCss>::Error>(())
    /// ```
    fn create<N: Borrow<str>, Css: TryParseCss>(classname: N, css: Css)
        -> Result<Self, Css::Error>;

    /// Creates a new style from some parsable css with a default prefix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylist::{Style, StyleExt};
    ///
    /// let style = Style::new("background-color: red;")?;
    /// # use stylist::TryParseCss;
    /// # Ok::<(), <&str as TryParseCss>::Error>(())
    /// ```
    fn new<Css: TryParseCss>(css: Css) -> Result<Self, Css::Error> {
        Self::create("stylist", css)
    }
}

impl StyleExt for Style {
    fn create<N: Borrow<str>, Css: TryParseCss>(
        class_prefix: N,
        css: Css,
    ) -> Result<Self, Css::Error> {
        let css = css.try_parse()?;
        Ok(Style::create_from_sheet(class_prefix, css))
    }
}
