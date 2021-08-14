use web_sys::window;

pub fn now() -> f64 {
    window().unwrap().performance().unwrap().now()
}
