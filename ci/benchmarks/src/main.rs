#[cfg(feature = "with_interface")]
mod app;
mod benchmarks;
mod runner;
mod utils;

#[cfg(feature = "with_interface")]
fn main() {
    use crate::app::App;
    use log::Level;

    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}

#[cfg(not(feature = "with_interface"))]
fn main() {
    use crate::runner::BenchmarkResults;
    use log::Level;

    console_log::init_with_level(Level::Info).expect("Failed to initialize Log!");
    let mut results = BenchmarkResults::default();
    while !results.step() {}

    log::info!("Benchmarks results: {:#?}", results);
}

#[cfg(test)]
mod tests {
    use crate::runner::BenchmarkResults;
    use log::Level;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn benchmark_runner() {
        console_log::init_with_level(Level::Debug).expect("");

        let mut results = BenchmarkResults::default();
        while results.step() {}

        log::debug!("Benchmarks results: {:#?}", results);
    }
}
