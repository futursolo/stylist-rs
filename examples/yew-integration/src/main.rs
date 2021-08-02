use stylist::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct CustomComponent {
    style: Style,
}

impl Component for CustomComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::new(
            r#"
                background-color: red;

                .on-da-inside {
                    background-color: blue;
                    width: 100px
                }
            "#,
        )
        .unwrap();
        CustomComponent { style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                {"The quick brown fox jumps over the lazy dog"}
                <div class="on-da-inside">{"The quick brown fox jumps over the lazy dog"}</div>
            </div>
        }
    }
}
fn main() {
    web_logger::init();
    yew::start_app::<CustomComponent>();
}
