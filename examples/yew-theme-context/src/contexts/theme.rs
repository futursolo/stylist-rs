use std::ops::Deref;
use std::rc::Rc;

use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> Rc<Theme> {
        thread_local! {
            static LIGHT_THEME: Rc<Theme> = Rc::new(Theme {
                font_color: "black".to_string(),
                background_color: "rgb(237, 244, 255)".to_string(),
                paper_color: "white".to_string(),
            });

            static DARK_THEME: Rc<Theme> = Rc::new(Theme {
                font_color: "white".to_string(),
                background_color: "black".to_string(),
                paper_color: "rgb(50, 50, 50)".to_string(),
            });
        }

        match self {
            ThemeKind::Dark => DARK_THEME.with(|m| m.clone()),
            ThemeKind::Light => LIGHT_THEME.with(|m| m.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: String,
    pub background_color: String,

    pub paper_color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind)
    }
}

impl Deref for ThemeContext {
    type Target = ThemeKind;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
