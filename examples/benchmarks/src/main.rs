use yew::prelude::*;

use log::Level;

mod benchmarks;
mod utils;

pub enum BenchMsg {
    ParseSimpleFinish(f64),
    ParseComplexFinish(f64),
    CachedLookupFinish(f64),
    CachedLookupBigSheetFinish(f64),
    MountingFinish(f64),
}

pub struct Benchmarks {
    link: ComponentLink<Self>,
    finished: bool,

    parse_simple: Option<f64>,
    parse_complex: Option<f64>,

    cached_lookup: Option<f64>,
    cached_lookup_big_sheet: Option<f64>,

    mounting: Option<f64>,
}

impl Component for Benchmarks {
    type Message = BenchMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            finished: false,
            parse_simple: None,
            parse_complex: None,
            cached_lookup: None,
            cached_lookup_big_sheet: None,
            mounting: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link
                .callback(|_| BenchMsg::ParseSimpleFinish(benchmarks::bench_parse_simple()))
                .emit(());
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BenchMsg::ParseSimpleFinish(m) => {
                self.parse_simple = Some(m);
                self.link
                    .callback(|_| BenchMsg::ParseComplexFinish(benchmarks::bench_parse_complex()))
                    .emit(());
            }
            BenchMsg::ParseComplexFinish(m) => {
                self.parse_complex = Some(m);
                self.link
                    .callback(|_| BenchMsg::CachedLookupFinish(benchmarks::bench_cached_lookup()))
                    .emit(());
            }

            BenchMsg::CachedLookupFinish(m) => {
                self.cached_lookup = Some(m);
                self.link
                    .callback(|_| {
                        BenchMsg::CachedLookupBigSheetFinish(
                            benchmarks::bench_cached_lookup_big_sheet(),
                        )
                    })
                    .emit(());
            }

            BenchMsg::CachedLookupBigSheetFinish(m) => {
                self.cached_lookup_big_sheet = Some(m);
                self.link
                    .callback(|_| BenchMsg::MountingFinish(benchmarks::bench_mounting()))
                    .emit(());
            }

            BenchMsg::MountingFinish(m) => {
                self.mounting = Some(m);
                self.finished = true;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {
                    if !self.finished {
                        html!{<div>{"Benchmarking... The browser may be unresponsive during the benchmark."}</div>}
                    } else {
                        Html::default()
                    }
                }
                <div>{"Parse Simple (100,000 iterations): "}{self.parse_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</div>
                <div>{"Parse Complex (10,000 iterations): "}{self.parse_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</div>
                <div>{"Cached Lookup (1,000,000 iterations): "}{self.cached_lookup.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</div>
                <div>{"Cached Lookup, Big Sheet (100,000 iterations): "}{self.cached_lookup_big_sheet.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</div>
                <div>{"Mounting (1,000 iterations): "}{self.mounting.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</div>
            </div>
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppMsg {
    Start,
}

pub struct App {
    link: ComponentLink<Self>,
    started: bool,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            started: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        assert_eq!(msg, AppMsg::Start);

        self.started = true;

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>

                {
                    if self.started {
                        html!{<Benchmarks />}
                    } else {
                        html!{<button onclick=self.link.callback(|_| AppMsg::Start)>
                            {"Start!"}
                        </button>}
                    }
                }
            </div>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
