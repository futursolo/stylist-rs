use gloo::timers::callback::Timeout;
use stylist::yew::Global;
use stylist::{StyleSource, YieldStyle};
use yew::prelude::*;

use crate::runner::BenchmarkResults;

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
    Step(BenchmarkResults),
}

pub struct Benchmarks {
    finished: bool,
    results: BenchmarkResults,
}

impl Benchmarks {
    fn step(&mut self, ctx: &Context<Benchmarks>) {
        let link = ctx.link();
        let mut results = self.results.clone();
        let cb = link.batch_callback_once(move |_| results.step().then(|| BenchMsg::Step(results)));
        Timeout::new(0, move || cb.emit(())).forget();
    }
}

impl Component for Benchmarks {
    type Message = BenchMsg;
    type Properties = ();

    fn create(_: &Context<Benchmarks>) -> Self {
        Self {
            finished: false,
            results: Default::default(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Benchmarks>, first_render: bool) {
        if first_render {
            self.step(ctx);
        }
    }

    fn update(&mut self, ctx: &Context<Benchmarks>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Step(next_results) => {
                self.results = next_results;
                self.step(ctx);
            }
        }
        true
    }

    fn changed(&mut self, _: &Context<Benchmarks>) -> bool {
        false
    }

    fn view(&self, _: &Context<Benchmarks>) -> Html {
        let results = &self.results;
        html! {
            <div class={self.style()}>
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
                            <th>{results.parse_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro (Literal) Simple (10,000,000 iterations): "}</th>
                            <th>{results.macro_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro (Inline) Simple (10,000,000 iterations): "}</th>
                            <th>{results.macro_inline_simple.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Parse Simple, No Cache (100,000 iterations): "}</th>
                            <th>{results.parse_simple_no_cache.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Parse Complex (1,000,000 iterations): "}</th>
                            <th>{results.parse_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro (Literal) Complex (1,000,000 iterations): "}</th>
                            <th>{results.macro_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Macro (Inline) Complex (1,000,000 iterations): "}</th>
                            <th>{results.macro_inline_complex.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Parse Complex, No Cache (100,000 iterations): "}</th>
                            <th>{results.parse_complex_no_cache.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Cached Lookup (1,000,000 iterations): "}</th>
                            <th>{results.cached_lookup.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Cached Lookup, Big Sheet (100,000 iterations): "}</th>
                            <th>{results.cached_lookup_big_sheet.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
                        </tr>
                        <tr>
                            <th>{"Mounting (2,000 iterations): "}</th>
                            <th>{results.mounting.map(|m| {format!("{:.0}ms", m)}).unwrap_or_else(|| "".to_string())}</th>
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
    started: bool,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: &Context<App>) -> Self {
        Self { started: false }
    }

    fn update(&mut self, _: &Context<App>, msg: Self::Message) -> bool {
        assert_eq!(msg, AppMsg::Start);

        self.started = true;

        true
    }

    fn changed(&mut self, _: &Context<App>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<App>) -> Html {
        html! {
            <>
                <Global css={GLOBAL_STYLE} />
                <div class={self.style()}>
                    <h1>{"Stylist Benchmark"}</h1>
                    {
                        if self.started {
                            html!{<Benchmarks />}
                        } else {
                            html!{
                                <>
                                    <div class="before-intro">{"To start benchmarking, please click start:"}</div>
                                    <button onclick={ctx.link().callback(|_| AppMsg::Start)}>
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
