use stylist::manager::StyleManager;
use stylist::yew::{styled_component, Global, ManagerProvider};
use yew::prelude::*;

#[styled_component]
pub fn Inside() -> Html {
    html! {
        <div class={css!(r#"
            width: 200px;
            height: 200px;
            border-radius: 5px;

            background: black;

            padding: 15px;
            box-sizing: border-box;

            box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
            color: white;
        "#)}>
            {"The quick brown fox jumps over the lazy dog"}
        </div>
    }
}

#[styled_component]
pub fn Content() -> Html {
    html! {
        <>
            // Global Styles can be applied with <Global /> component.
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
            <h1>{"Yew Integration"}</h1>
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
                {"The quick brown fox jumps over the lazy dog"}
                <Inside />
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ServerAppProps {
    pub manager: StyleManager,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let fallback = html! { <div>{"Loading..."}</div> };

    html! {
        <Suspense {fallback}>
            <ManagerProvider manager={props.manager.clone()}>
                <Content />
            </ManagerProvider>
        </Suspense>
    }
}

#[function_component]
pub fn App() -> Html {
    let fallback = html! { <div>{"Loading..."}</div> };
    let style_mgr = (*use_memo(
        |_| StyleManager::new().expect("failed to create style manager."),
        (),
    ))
    .to_owned();

    html! {
        <Suspense {fallback}>
            <ManagerProvider manager={style_mgr}>
                <Content />
            </ManagerProvider>
        </Suspense>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use std::time::Duration;
    use stylist::manager::render_static;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use web_sys::window;

    #[wasm_bindgen_test]
    async fn test_simple() {
        let (writer, mut reader) = render_static();
        let manager = StyleManager::builder()
            .writer(writer)
            .build()
            .expect("failed to create style manager.");

        let body_s = yew::LocalServerRenderer::<ServerApp>::with_props(ServerAppProps { manager })
            .render()
            .await;

        let head_s = reader
            .read_static_markup()
            .await
            .expect("failed to read styles.");

        // No styles are rendered to head element during SSR.
        assert_eq!(
            gloo_utils::document()
                .query_selector_all("[data-style]")
                .unwrap()
                .length(),
            0
        );

        let frag = document().create_document_fragment();
        frag.set_node_value(Some(head_s));

        // Manually append styles.
        gloo_utils::head().append_child(&frag).unwrap();

        let output_el = gloo_utils::document().get_element_by_id("output").unwrap();
        output_el.set_inner_html(&body_s);

        yew::Renderer::<App>::with_root(output_el).render();
        // wait for lifecycles to process
        yew::platform::time::sleep(Duration::from_millis(50)).await;

        // There should be 3 style elements (SSR ones)
        assert_eq!(
            gloo_utils::document()
                .query_selector_all("[data-style]")
                .unwrap()
                .length(),
            3
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
    }
}
