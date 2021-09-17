use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let mgr = __stylist_style_manager__.clone();

    Html::default()
}

fn main() {}
