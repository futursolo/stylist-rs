use stylist::yew::{styled_component, use_media_query, Global};
use yew::prelude::*;

use log::Level;

#[styled_component(App)]
pub fn app() -> Html {
    let is_small = use_media_query("(max-width: 720px)");

    let size_name = if is_small { "small" } else { "big" };

    html! {
        <>
            <Global css={css!(r#"
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
                "#)} />
            <h1>{"Use Media Query Example"}</h1>
            <div class={css!(r#"
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
            "#)} id="yew-sample-content">
                {"To start, try adjust your browser width."}
                <br />
                {"You are now using the: "}{size_name}{" Window."}
            </div>
        </>
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use web_sys::window;

    #[wasm_bindgen_test]
    fn test_simple() {
        yew::Renderer::<App>::with_root(
            gloo_utils::document().get_element_by_id("output").unwrap(),
        )
        .render();
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
    }
}
