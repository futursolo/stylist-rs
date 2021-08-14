use std::sync::Arc;

use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

#[derive(Debug, Clone)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

#[derive(Debug)]
pub(crate) struct Theme {
    pub font_color: String,
    pub background_color: String,

    pub paper_color: String,
}

#[derive(Debug)]
pub(crate) enum Action {
    SetTheme(ThemeKind),
}

pub(crate) struct ThemeStore {
    pub kind: ThemeKind,

    light_theme: Arc<Theme>,
    dark_theme: Arc<Theme>,
}

impl Store for ThemeStore {
    type Action = Action;
    type Input = Action;

    fn new() -> Self {
        Self {
            kind: ThemeKind::Light,

            light_theme: Theme {
                font_color: "black".to_string(),
                background_color: "rgb(237, 244, 255)".to_string(),
                paper_color: "white".to_string(),
            }
            .into(),

            dark_theme: Theme {
                font_color: "white".to_string(),
                background_color: "black".to_string(),
                paper_color: "rgb(50, 50, 50)".to_string(),
            }
            .into(),
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Self::Input::SetTheme(kind) => {
                link.send_message(Action::SetTheme(kind));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Self::Action::SetTheme(kind) => {
                self.kind = kind;
            }
        }
    }
}

impl ThemeStore {
    pub fn current(&self) -> Arc<Theme> {
        match self.kind {
            ThemeKind::Dark => self.dark_theme.clone(),
            ThemeKind::Light => self.light_theme.clone(),
        }
    }
}
