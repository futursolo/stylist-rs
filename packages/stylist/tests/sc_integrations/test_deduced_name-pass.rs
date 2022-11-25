use stylist::yew::styled_component;
use yew::prelude::*;

// See issue #90: get component identifier from function name
#[styled_component]
pub fn App() -> Html {
    let _ = css!( background-color: grey; );
    Html::default()
}

fn main() {
    let _ = html! { <App /> };
}
