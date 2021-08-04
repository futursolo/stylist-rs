use yewdux::prelude::*;

pub(crate) mod theme;

use theme::{ThemeKind, ThemeStore};

pub(crate) enum Action {
    SetTheme(ThemeKind),
}

#[derive(Default, Clone)]
pub(crate) struct Store {
    pub theme: ThemeStore,
}

impl Reducer for Store {
    type Action = Action;

    fn new() -> Self {
        Self::default()
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::SetTheme(m) => self.theme.set_theme(m),
        }

        true
    }
}

pub(crate) type AppStore = ReducerStore<Store>;

pub(crate) type AppDispatch = DispatchProps<AppStore>;
