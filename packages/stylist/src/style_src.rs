#[cfg(feature = "parser")]
use std::borrow::Cow;

#[cfg(not(feature = "parser"))]
use std::marker::PhantomData;

use crate::ast::Sheet;
use crate::Result;
#[cfg(feature = "yew_integration")]
use crate::Style;

#[cfg(feature = "parser")]
#[derive(Debug, Clone, PartialEq)]
enum SheetSource<'a> {
    String(Cow<'a, str>),
    Sheet(Sheet),
}

#[cfg(not(feature = "parser"))]
#[derive(Debug, Clone, PartialEq)]
enum SheetSource {
    Sheet(Sheet),
}

/// A struct that can be used as a source to create a [`Style`](crate::Style) or
/// [`GlobalStyle`](crate::GlobalStyle).
///
/// This struct is usually created by [`css!`](crate::css) macro.
///
/// You can also get a StyleSource instance from a string or a [`Sheet`] by calling `.into()`.
///
/// ```rust
/// use stylist::StyleSource;
/// use yew::prelude::*;
/// use stylist::yew::Global;
///
/// let s: StyleSource = "color: red;".into();
///
/// let rendered = html! {<div class=s.clone() />};
///
/// let global_rendered = html! {<Global css=s />};
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSource<'a> {
    #[cfg(feature = "parser")]
    inner: SheetSource<'a>,

    #[cfg(not(feature = "parser"))]
    inner: SheetSource,
    #[cfg(not(feature = "parser"))]
    _marker: PhantomData<&'a ()>,
}

impl StyleSource<'_> {
    pub fn try_to_sheet(&self) -> Result<Sheet> {
        match self.inner {
            SheetSource::Sheet(ref m) => Ok(m.clone()),
            #[cfg_attr(documenting, doc(cfg(feature = "parser")))]
            #[cfg(feature = "parser")]
            SheetSource::String(ref m) => m.parse::<Sheet>(),
        }
    }

    #[cfg_attr(documenting, doc(cfg(feature = "yew_integration")))]
    #[cfg(feature = "yew_integration")]
    pub(crate) fn to_style(&self) -> Style {
        Style::new(self.clone()).expect("Failed to create style")
    }
}

impl From<Sheet> for StyleSource<'_> {
    fn from(other: Sheet) -> StyleSource<'static> {
        StyleSource {
            inner: SheetSource::Sheet(other),
            #[cfg(not(feature = "parser"))]
            _marker: PhantomData,
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use super::*;

    impl From<String> for StyleSource<'_> {
        fn from(other: String) -> StyleSource<'static> {
            StyleSource {
                inner: SheetSource::String(other.into()),
            }
        }
    }

    impl<'a> From<&'a str> for StyleSource<'a> {
        fn from(other: &'a str) -> StyleSource<'a> {
            StyleSource {
                inner: SheetSource::String(other.into()),
            }
        }
    }

    impl<'a> From<Cow<'a, str>> for StyleSource<'a> {
        fn from(other: Cow<'a, str>) -> StyleSource<'a> {
            StyleSource {
                inner: SheetSource::String(other),
            }
        }
    }
}
