use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: String,
    pub background_color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeStore {
    pub kind: ThemeKind,

    light_theme: Arc<Theme>,
    dark_theme: Arc<Theme>,
}

impl Default for ThemeStore {
    fn default() -> Self {
        Self {
            kind: ThemeKind::Light,

            light_theme: Theme {
                font_color: "black".to_string(),
                background_color: "white".to_string(),
            }
            .into(),

            dark_theme: Theme {
                font_color: "white".to_string(),
                background_color: "black".to_string(),
            }
            .into(),
        }
    }
}

impl ThemeStore {
    pub fn set_theme(&mut self, kind: ThemeKind) {
        self.kind = kind;
    }

    pub fn current(&self) -> Arc<Theme> {
        match self.kind {
            ThemeKind::Dark => self.dark_theme.clone(),
            ThemeKind::Light => self.light_theme.clone(),
        }
    }
}
