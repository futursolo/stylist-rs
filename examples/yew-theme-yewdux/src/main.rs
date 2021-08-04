use std::borrow::Cow;

use stylist::YieldStyle;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewdux::prelude::*;
use yewtil::NeqAssign;

use log::Level;

mod store;

use store::theme::ThemeKind;
use store::{Action, AppDispatch};

pub(crate) struct BaseInside {
    dispatch: AppDispatch,
}

impl Component for BaseInside {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style()>
                {"The quick brown fox jumps over the lazy dog"}
            </div>
        }
    }
}

impl YieldStyle for BaseInside {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.dispatch.state().theme.current();

        format!(
            r#"
                color: {font_color};
            "#,
            font_color = theme.font_color
        )
        .into()
    }
}

pub(crate) type Inside = WithDispatch<BaseInside>;

pub(crate) struct App {
    dispatch: AppDispatch,
}

impl Component for App {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let theme_kind = self.dispatch.state().theme.kind.clone();

        let other_theme = match theme_kind {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };

        let toggle_theme = self
            .dispatch
            .callback(move |_| Action::SetTheme(other_theme.clone()));

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
        let theme = self.dispatch.state().theme.current();

        format!(
            r#"
                height: 100vh;
                width: 100vw;
                background-color: {bg_color};
            "#,
            bg_color = theme.background_color
        )
        .into()
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<WithDispatch<App>>();
}
