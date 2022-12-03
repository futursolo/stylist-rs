use example_yew_ssr::App;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::Renderer::<App>::new().hydrate();
}
