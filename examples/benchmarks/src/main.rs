use gloo::timers::callback::Timeout;
use stylist::{yew::Global, StyleSource, YieldStyle};
use yew::prelude::*;

use log::Level;

mod benchmarks;
mod utils;

static GLOBAL_STYLE: &str = r#"
    html, body {
        margin: 0;
        padding: 0;
        font-family: sans-serif;
        display: flex;
        justify-content: center;
        flex-direction: column;
        align-items: center;
    }
"#;

pub enum BenchMsg {
    ParseSimpleFinish(f64),
    MacroSimpleFinish(f64),
    MacroInlineSimpleFinish(f64),
    ParseSimpleNoCacheFinish(f64),
    ParseComplexFinish(f64),
    MacroComplexFinish(f64),
    MacroInlineComplexFinish(f64),
    ParseComplexNoCacheFinish(f64),
    CachedLookupFinish(f64),
    CachedLookupBigSheetFinish(f64),
    MountingFinish(f64),
}

pub struct Benchmarks {
    link: ComponentLink<Self>,
    finished: bool,

    parse_simple: Option<f64>,
    macro_simple: Option<f64>,
    macro_inline_simple: Option<f64>,
    parse_simple_no_cache: Option<f64>,
    parse_complex: Option<f64>,
    macro_complex: Option<f64>,
    macro_inline_complex: Option<f64>,
    parse_complex_no_cache: Option<f64>,

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
            macro_simple: None,
            macro_inline_simple: None,
            parse_simple_no_cache: None,
            parse_complex: None,
            macro_complex: None,
            macro_inline_complex: None,
            parse_complex_no_cache: None,
            cached_lookup: None,
            cached_lookup_big_sheet: None,
            mounting: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let cb = self
                .link
                .callback(|_| BenchMsg::ParseSimpleFinish(benchmarks::bench_parse_simple()));
            Timeout::new(100, move || cb.emit(())).forget();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BenchMsg::ParseSimpleFinish(m) => {
                self.parse_simple = Some(m);
                let cb = self
                    .link
                    .callback(|_| BenchMsg::MacroSimpleFinish(benchmarks::bench_macro_simple()));

                Timeout::new(100, move || cb.emit(())).forget();
            }
            BenchMsg::MacroSimpleFinish(m) => {
                self.macro_simple = Some(m);
                let cb = self.link.callback(|_| {
                    BenchMsg::MacroInlineSimpleFinish(benchmarks::bench_macro_inline_simple())
                });

                Timeout::new(100, move || cb.emit(())).forget();
            }
            BenchMsg::MacroInlineSimpleFinish(m) => {
                self.macro_inline_simple = Some(m);
                let cb = self.link.callback(|_| {
                    BenchMsg::ParseSimpleNoCacheFinish(benchmarks::bench_parse_simple_no_cache())
                });

                Timeout::new(100, move || cb.emit(())).forget();
            }
            BenchMsg::ParseSimpleNoCacheFinish(m) => {
                self.parse_simple_no_cache = Some(m);
                let cb = self
                    .link
                    .callback(|_| BenchMsg::ParseComplexFinish(benchmarks::bench_parse_complex()));

                Timeout::new(100, move || cb.emit(())).forget();
            }

            BenchMsg::ParseComplexFinish(m) => {
                self.parse_complex = Some(m);

                let cb = self
                    .link
                    .callback(|_| BenchMsg::MacroComplexFinish(benchmarks::bench_macro_complex()));

                Timeout::new(100, move || cb.emit(())).forget();
            }
            BenchMsg::MacroComplexFinish(m) => {
                self.macro_complex = Some(m);

                let cb = self.link.callback(|_| {
                    BenchMsg::MacroInlineComplexFinish(benchmarks::bench_macro_inline_complex())
                });

                Timeout::new(100, move || cb.emit(())).forget();
            }
            BenchMsg::MacroInlineComplexFinish(m) => {
                self.macro_inline_complex = Some(m);

                let cb = self.link.callback(|_| {
                    BenchMsg::ParseComplexNoCacheFinish(benchmarks::bench_parse_complex_no_cache())
                });

                Timeout::new(100, move || cb.emit(())).forget();
            }
            BenchMsg::ParseComplexNoCacheFinish(m) => {
                self.parse_complex_no_cache = Some(m);

                let cb = self
                    .link
                    .callback(|_| BenchMsg::CachedLookupFinish(benchmarks::bench_cached_lookup()));

                Timeout::new(100, move || cb.emit(())).forget();
            }

            BenchMsg::CachedLookupFinish(m) => {
                self.cached_lookup = Some(m);

                let cb =
                    self.link.callback(|_| {
                        BenchMsg::CachedLookupBigSheetFinish(
                            benchmarks::bench_cached_lookup_big_sheet(),
                        )
                    });

                Timeout::new(100, move || cb.emit(())).forget();
            }

            BenchMsg::CachedLookupBigSheetFinish(m) => {
                self.cached_lookup_big_sheet = Some(m);

                let cb = self
                    .link
                    .callback(|_| BenchMsg::MountingFinish(benchmarks::bench_mounting()));

                Timeout::new(100, move || cb.emit(())).forget();
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
            <div class=self.style()>
                {
                    if !self.finished {
                        html!{<div class="running">{"Benchmarking..."}<br />{"The browser may be unresponsive during the benchmark."}</div>}
                    } else {
                        html!{<div class="running" />}
                    }
                }

                <table>
                    <thead>
                        <tr>
                            <th>{"Benchmark"}</th>
                            <th>{"Result"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <th>{"Parse Simple (10,000,000 iterations): "}</th>
                            <th>{self.parse_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro Simple (10,000,000 iterations): "}</th>
                            <th>{self.macro_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro Inline Simple (10,000,000 iterations): "}</th>
                            <th>{self.macro_inline_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Parse Simple, No Cache (100,000 iterations): "}</th>
                            <th>{self.parse_simple_no_cache.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Parse Complex (1,000,000 iterations): "}</th>
                            <th>{self.parse_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro Complex (1,000,000 iterations): "}</th>
                            <th>{self.macro_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro Inline Complex (1,000,000 iterations): "}</th>
                            <th>{self.macro_inline_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Parse Complex, No Cache (100,000 iterations): "}</th>
                            <th>{self.parse_complex_no_cache.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Cached Lookup (1,000,000 iterations): "}</th>
                            <th>{self.cached_lookup.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Cached Lookup, Big Sheet (100,000 iterations): "}</th>
                            <th>{self.cached_lookup_big_sheet.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Mounting (1,000 iterations): "}</th>
                            <th>{self.mounting.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }
}

impl YieldStyle for Benchmarks {
    fn style_from(&self) -> StyleSource<'static> {
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;

            .running {
                height: 50px;
            }

            table {
                border: 1px solid black;
                border-collapse: collapse;
            }

            thead {
                font-weight: bold;
                background-color: rgb(240, 240, 240);
            }

            th {
                text-align: left;
                border: 1px solid black;
                border-collapse: collapse;
                padding: 5px;
            }

            tbody th {
                font-weight: normal;
            }

            th:nth-child(1) {
                padding-right: 20px;
            }

            th:nth-child(2) {
                padding-left: 20px;
                padding-right: 20px;
            }

            tbody tr:nth-child(even) {
                background-color: rgb(240, 240, 240);
            }
        "#
        .into()
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
            <>
                <Global css=GLOBAL_STYLE />
                <div class=self.style()>
                    <h1>{"Stylist Benchmark"}</h1>
                    {
                        if self.started {
                            html!{<Benchmarks />}
                        } else {
                            html!{
                                <>
                                    <div class="before-intro">{"To start benchmarking, please click start:"}</div>
                                    <button onclick=self.link.callback(|_| AppMsg::Start)>
                                        {"Start!"}
                                    </button>
                                </>
                            }
                        }
                    }
                </div>
            </>
        }
    }
}

impl YieldStyle for App {
    fn style_from(&self) -> StyleSource<'static> {
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;

            .before-intro {
                padding-bottom: 20px;
            }

            button {
                width: 300px;
                height: 50px;
                font-size: 20px;
            }
        "#
        .into()
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
