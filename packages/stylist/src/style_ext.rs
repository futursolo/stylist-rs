
use stylist_core::Style;
use std::borrow::Borrow;
use stylist_core::ast::Scopes;
use std::result::Result;

pub trait TryParseCss {
    type Error;
    fn try_parse(self) -> Result<Scopes, Self::Error>;
}

impl<'a> TryParseCss for &'a str {
    type Error = crate::error::Error;

    fn try_parse(self) -> Result<Scopes, Self::Error> {
        crate::parser::Parser::parse(self)
    }
}

pub trait StyleExt: Sized {
    fn create<N: Borrow<str>, Css: TryParseCss>(
        classname: N,
        css: Css,
    ) -> Result<Self, Css::Error>;

    fn new<Css: TryParseCss>(
        css: Css,
    ) -> Result<Self, Css::Error> {
        Self::create("stylist", css)
    }
}

impl StyleExt for Style {
    fn create<N: Borrow<str>, Css: TryParseCss>(
        class_prefix: N,
        css: Css,
    ) -> Result<Self, Css::Error> {
        let css = css.try_parse()?;
        Ok(Style::create_from_scopes(class_prefix, css))
    }
}