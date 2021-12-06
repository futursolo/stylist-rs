use stylist::yew::{styled_component, Global};
use yew::prelude::*;

use log::Level;

mod contexts;

use contexts::{use_theme, ThemeKind, ThemeProvider};

#[styled_component(Inside)]
pub fn inside() -> Html {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme",
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));

    html! {
        <div>
            <button class={css!(r#"color: white;
                height: 50px;
                width: 300px;
                font-size: 20px;
                background-color: rgb(88, 164, 255);
                border-radius: 5px;
                border: none;
            "#)} onclick={switch_theme} id="yew-sample-button">{"Switch to "}{theme_str}</button>
        </div>
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "light theme",
        ThemeKind::Dark => "dark theme",
    };

    html! {
        <>
            // Global Styles can be applied with <Global /> component.
            <Global css={css!(
                r#"
                    html, body {
                        font-family: sans-serif;

                        padding: 0;
                        margin: 0;

                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;

                        background-color: ${bg};
                        color: ${ft_color};
                    }
                "#,
                bg = theme.background_color.clone(),
                ft_color = theme.font_color.clone(),
            )} />
            <h1>{"Yew Theming w/ Context"}</h1>
            <div class={css!(
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
                    background-color: ${bg};
                "#,
                bg = theme.paper_color.clone()
            )} id="yew-sample-content">
                {"You are now using the "}{theme_str}{"!"}
                <Inside />
            </div>
        </>
    }
}

#[styled_component(Root)]
pub fn root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<Root>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use web_sys::window;

    #[wasm_bindgen_test]
    fn test_simple() {
        yew::start_app_in_element::<App>(
            gloo_utils::document().get_element_by_id("output").unwrap(),
        );
        let window = window().unwrap();
        let doc = window.document().unwrap();
        let body = window.document().unwrap().body().unwrap();

        let content = doc.query_selector("#yew-sample-content").unwrap().unwrap();

        let body_style = window.get_computed_style(&body).unwrap().unwrap();
        let content_style = window.get_computed_style(&content).unwrap().unwrap();

        let bg_color = body_style.get_property_value("background-color").unwrap();
        assert_eq!(bg_color, "rgb(237, 244, 255)");

        let content_display = content_style.get_property_value("display").unwrap();
        assert_eq!(content_display, "flex");

        let button = doc
            .query_selector("#yew-sample-button")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        button.click();

        let bg_color = body_style.get_property_value("background-color").unwrap();
        assert_eq!(bg_color, "rgb(0, 0, 0)");
    }
}
