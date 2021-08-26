use stylist::yew::Global;
use stylist::{Style, StyleSource, YieldStyle};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use log::Level;

pub struct Inside {}

impl Component for Inside {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
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
    fn style_from(&self) -> StyleSource<'static> {
        r#"
            width: 200px;
            height: 200px;
            border-radius: 5px;

            background: black;

            padding: 15px;
            box-sizing: border-box;

            box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
            color: white;

        "#
        .into()
    }
}

pub struct App {
    style: Style,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Alternatively, you can create Style manually during Component creation.
        let style = Style::new(
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
                background-color: white;
            "#,
        )
        .unwrap();
        Self { style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                // Global Styles can be applied with <GlobalStyle /> component.
                <Global css=r#"
                    html, body {
                        font-family: sans-serif;

                        padding: 0;
                        margin: 0;

                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;

                        background-color: rgb(237, 244, 255);
                    }
                "# />
                <h1>{"Yew Integration"}</h1>
                <div class=self.style.clone()>
                    {"The quick brown fox jumps over the lazy dog"}
                    <Inside />
                </div>
            </>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
