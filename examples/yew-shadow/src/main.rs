use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

use stylist::manager::StyleManager;
use stylist::registry::StyleRegistry;
use stylist::yew::GlobalStyle;
use stylist::{Result, Style};
use web_sys::{window, Element, Node, ShadowRootInit, ShadowRootMode};
use yew::prelude::*;

use log::Level;

/// The default Style Manager.
#[derive(Clone, Debug)]
pub(crate) struct ShadowManager {
    node: Node,
}

impl StyleManager for ShadowManager {
    fn get_registry(&self) -> Rc<RefCell<StyleRegistry>> {
        thread_local! {
            static REGISTRY: Lazy<Rc<RefCell<StyleRegistry>>> = Lazy::new(Rc::default);
        }

        REGISTRY.with(|m| (*m).clone())
    }

    fn get_container(&self) -> Result<Node> {
        Ok(self.node.clone())
    }
}

pub struct ShadowRoot {
    root_ref: NodeRef,
}

impl Component for ShadowRoot {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            root_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(m) = self.root_ref.cast::<Element>() {
                if let Ok(root) = m.attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open)) {
                    let mgr = ShadowManager {
                        node: root.clone().into(),
                    };

                    let style = Style::new_with_manager("background-color: pink;", mgr).unwrap();

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

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div ref=self.root_ref.clone() />
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
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
                <GlobalStyle css=r#"
                    span {
                        color: red;
                    }
                "# />
                <div>
                    <span>{"Outside of Shadow DOM."}</span>
                    <ShadowRoot />
                </div>
            </>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
