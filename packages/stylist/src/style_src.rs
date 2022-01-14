#[cfg(feature = "parser")]
use std::borrow::Cow;
#[cfg(not(feature = "parser"))]
use std::marker::PhantomData;

use crate::ast::Sheet;
use crate::manager::StyleManager;
use crate::Result;
#[cfg(feature = "yew")]
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
/// let rendered = html! {<div class={s.clone()} />};
/// let global_rendered = html! {<Global css={s} />};
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSource<'a> {
    #[cfg(feature = "parser")]
    inner: SheetSource<'a>,

    #[cfg(not(feature = "parser"))]
    inner: SheetSource,
    #[cfg(not(feature = "parser"))]
    _marker: PhantomData<&'a ()>,

    manager: Option<StyleManager>,
    #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
    pub(crate) location: String,
}

impl StyleSource<'_> {
    #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
    #[track_caller]
    pub(crate) fn get_caller_location() -> String {
        let caller_loc = std::panic::Location::caller();
        // Who cares if this a valid css class name, it's debugging info
        format!(
            "{}:{}:{}",
            caller_loc.file(),
            caller_loc.line(),
            caller_loc.column()
        )
    }

    pub(crate) fn try_into_sheet(self) -> Result<Sheet> {
        match self.inner {
            SheetSource::Sheet(m) => Ok(m),
            #[cfg(feature = "parser")]
            SheetSource::String(m) => Ok(m.parse::<Sheet>()?),
        }
    }

    #[cfg(feature = "yew")]
    pub(crate) fn into_style(mut self) -> Style {
        use stylist_core::ResultDisplay;
        let manager = self.manager.take().unwrap_or_default();
        Style::new_with_manager(self, manager).expect_display("Failed to create style")
    }

    #[doc(hidden)]
    pub fn with_manager(mut self, manager: StyleManager) -> Self {
        self.manager = Some(manager);

        self
    }
}

impl From<Sheet> for StyleSource<'_> {
    #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
    fn from(other: Sheet) -> StyleSource<'static> {
        StyleSource {
            inner: SheetSource::Sheet(other),
            #[cfg(not(feature = "parser"))]
            _marker: PhantomData,
            manager: None,
            #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
            location: Self::get_caller_location(),
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use super::*;

    impl From<String> for StyleSource<'_> {
        #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
        fn from(other: String) -> StyleSource<'static> {
            StyleSource {
                inner: SheetSource::String(other.into()),
                manager: None,
                #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
                location: Self::get_caller_location(),
            }
        }
    }

    impl<'a> From<&'a str> for StyleSource<'a> {
        #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
        fn from(other: &'a str) -> StyleSource<'a> {
            StyleSource {
                inner: SheetSource::String(other.into()),
                manager: None,
                #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
                location: Self::get_caller_location(),
            }
        }
    }

    impl<'a> From<Cow<'a, str>> for StyleSource<'a> {
        #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
        fn from(other: Cow<'a, str>) -> StyleSource<'a> {
            StyleSource {
                inner: SheetSource::String(other),
                manager: None,
                #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
                location: Self::get_caller_location(),
            }
        }
    }
}
