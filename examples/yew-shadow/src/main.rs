use stylist::manager::StyleManager;
use stylist::yew::Global;
use stylist::Style;
use web_sys::{window, Element, ShadowRootInit, ShadowRootMode};
use yew::prelude::*;

use log::Level;

pub struct ShadowRoot {
    root_ref: NodeRef,
}

impl Component for ShadowRoot {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            root_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(m) = self.root_ref.cast::<Element>() {
                if let Ok(root) = m.attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open)) {
                    let mgr = StyleManager::builder()
                        .container(root.clone().into())
                        .build()
                        .expect("Failed to create manager.");

                    let style = Style::new_with_manager(
                        r#"
                            background-color: pink;
                            width: 200px;
                            height: 200px;
                            border-radius: 5px;


                            padding: 15px;
                            box-sizing: border-box;

                            box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                        "#,
                        mgr,
                    )
                    .unwrap();

                    let children = window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_element("div")
                        .unwrap();
                    children.set_inner_html(
                        format!(
                            "<div class=\"{}\"><span>Inside Shadow DOM.</span></div>",
                            style.get_class_name()
                        )
                        .as_str(),
                    );

                    root.append_child(&children).unwrap();
                }
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.root_ref.clone()} />
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let card = stylist::css!(
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
            "#
        );
        html! {
            <>
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

                    span {
                        color: red;
                    }
                "# />
                <h1>{"Yew Shadow DOM Example"}</h1>
                <div class={card}>
                    <span>{"Outside of Shadow DOM."}</span>
                    <ShadowRoot />
                </div>
            </>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::Renderer::<App>::new().render();
}
