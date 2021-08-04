use std::borrow::Cow;
use std::sync::Arc;

use stylist::YieldStyle;
use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use log::Level;

mod theme;

use theme::{Theme, ThemeKind, ThemeStore};

pub(crate) enum InsideMsg {
    ThemeUpdated(ReadOnly<ThemeStore>),
}

pub(crate) struct Inside {
    theme: Option<Arc<Theme>>,
    _theme_store: Box<dyn Bridge<StoreWrapper<ThemeStore>>>,
}

impl Component for Inside {
    type Message = InsideMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(InsideMsg::ThemeUpdated);
        Self {
            theme: None,
            _theme_store: ThemeStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            InsideMsg::ThemeUpdated(m) => {
                let m = m.borrow();
                self.theme = Some(m.current());
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style()>
                {"The quick brown fox jumps over the lazy dog"}
            </div>
        }
    }
}

// You can implement YieldStyle for your component and call `.style()` in the render method.
impl YieldStyle for Inside {
    // Every `.style()` is called, this method will also be called.
    // So you can create style dynamically (theming).
    fn style_str(&self) -> Cow<'static, str> {
        if let Some(ref m) = self.theme {
            format!(
                r#"
                    color: {font_color};
                "#,
                font_color = m.font_color
            )
            .into()
        } else {
            "".into()
        }
    }
}

pub(crate) enum AppMsg {
    SetTheme(ThemeKind),
    ThemeUpdated(ReadOnly<ThemeStore>),
}

pub(crate) struct App {
    link: ComponentLink<Self>,
    theme: Option<Arc<Theme>>,
    theme_kind: ThemeKind,
    theme_store: Box<dyn Bridge<StoreWrapper<ThemeStore>>>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(AppMsg::ThemeUpdated);

        Self {
            link,
            theme: None,
            theme_kind: ThemeKind::Light,
            theme_store: ThemeStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::ThemeUpdated(m) => {
                let m = m.borrow();
                self.theme = Some(m.current());
                self.theme_kind = m.kind.clone();
            }
            AppMsg::SetTheme(m) => {
                self.theme_store.send(theme::Action::SetTheme(m));
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let other_theme = match self.theme_kind {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };

        let toggle_theme = self
            .link
            .callback(move |_| AppMsg::SetTheme(other_theme.clone()));

        html! {
            <div class=self.style()>
                <Inside />
                <button onclick=toggle_theme>{"Toggle Theme"}</button>
            </div>
        }
    }
}

impl YieldStyle for App {
    fn style_str(&self) -> Cow<'static, str> {
        if let Some(ref m) = self.theme {
            format!(
                r#"
                    height: 100vh;
                    width: 100vw;
                    background-color: {bg_color};
                "#,
                bg_color = m.background_color
            )
            .into()
        } else {
            "".into()
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
