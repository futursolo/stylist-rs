use std::borrow::Cow;
use std::sync::Arc;

use stylist::yew::GlobalStyle;
use stylist::YieldStyle;
use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use log::Level;

mod theme;

use theme::{Theme, ThemeKind, ThemeStore};

pub(crate) enum InsideMsg {
    SetTheme(ThemeKind),
    ThemeUpdated(ReadOnly<ThemeStore>),
}

pub(crate) struct Inside {
    link: ComponentLink<Self>,
    theme_kind: ThemeKind,
    theme: Option<Arc<Theme>>,
    theme_store: Box<dyn Bridge<StoreWrapper<ThemeStore>>>,
}

impl Component for Inside {
    type Message = InsideMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(InsideMsg::ThemeUpdated);
        Self {
            link,
            theme_kind: ThemeKind::Light,
            theme: None,
            theme_store: ThemeStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            InsideMsg::ThemeUpdated(m) => {
                let m = m.borrow();
                self.theme_kind = m.kind.clone();
                self.theme = Some(m.current());
            }
            InsideMsg::SetTheme(m) => {
                self.theme_store.send(theme::Action::SetTheme(m));
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let theme_str = match self.theme_kind {
            ThemeKind::Light => "Dark Theme",
            ThemeKind::Dark => "Light Theme",
        };

        let other_theme = match self.theme_kind {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };

        let switch_theme = self
            .link
            .callback(move |_| InsideMsg::SetTheme(other_theme.clone()));

        html! {
            <div class=self.style()>
                <button onclick=switch_theme>{"Switch to "}{theme_str}</button>
            </div>
        }
    }
}

impl YieldStyle for Inside {
    fn style_str(&self) -> Cow<'static, str> {
        r#"
            button {
                color: white;
                height: 50px;
                width: 300px;
                font-size: 20px;
                background-color: rgb(88, 164, 255);
                border-radius: 5px;
                border: none;
            }
        "#
        .into()
    }
}

pub(crate) enum AppMsg {
    ThemeUpdated(ReadOnly<ThemeStore>),
}

pub(crate) struct App {
    theme: Option<Arc<Theme>>,
    theme_kind: ThemeKind,
    _theme_store: Box<dyn Bridge<StoreWrapper<ThemeStore>>>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(AppMsg::ThemeUpdated);

        Self {
            theme: None,
            theme_kind: ThemeKind::Light,
            _theme_store: ThemeStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::ThemeUpdated(m) => {
                let m = m.borrow();
                self.theme = Some(m.current());
                self.theme_kind = m.kind.clone();
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.theme.is_none() {
            return Html::default();
        }

        let theme = self.theme.clone().unwrap();

        let theme_str = match self.theme_kind {
            ThemeKind::Light => "light theme",
            ThemeKind::Dark => "dark theme",
        };

        html! {
            <>
                // Global Styles can be applied with <GlobalStyle /> component.
                <GlobalStyle css=format!(
                    r#"
                        &, & body {{
                            font-family: sans-serif;

                            padding: 0;
                            margin: 0;

                            display: flex;
                            justify-content: center;
                            align-items: center;
                            min-height: 100vh;
                            flex-direction: column;

                            background-color: {bg};
                            color: {ft_color};
                        }}
                    "#,
                    bg = theme.background_color,
                    ft_color = theme.font_color,
                ) />
                <h1>{"Yew Theming w/ Agent"}</h1>
                <div class=self.style()>
                    {"You are now using the "}{theme_str}{"!"}
                    <Inside />
                </div>
            </>
        }

        // let toggle_theme = self
        //     .link
        //     .callback(move |_| AppMsg::SetTheme(other_theme.clone()));

        // html! {
        //     <div class=self.style()>
        //         <Inside />
        //         <button onclick=toggle_theme>{"Toggle Theme"}</button>
        //     </div>
        // }
    }
}

impl YieldStyle for App {
    fn style_str(&self) -> Cow<'static, str> {
        if let Some(ref m) = self.theme {
            format!(
                r#"
                    box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                    height: 500px;
                    width: 500px;
                    border-radius: 5px;

                    display: flex;
                    justify-content: space-around;
                    align-items: center;

                    padding: 15px;
                    box-sizing: border-box;

                    flex-direction: column;
                    background-color: {bg};
                "#,
                bg = m.paper_color
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
