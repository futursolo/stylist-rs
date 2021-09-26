use crate::benchmarks;

#[derive(Default, Clone, Debug)]
pub struct BenchmarkResults {
    pub parse_simple: Option<f64>,
    pub macro_simple: Option<f64>,
    pub macro_inline_simple: Option<f64>,
    pub parse_simple_no_cache: Option<f64>,
    pub parse_complex: Option<f64>,
    pub macro_complex: Option<f64>,
    pub macro_inline_complex: Option<f64>,
    pub parse_complex_no_cache: Option<f64>,

    pub cached_lookup: Option<f64>,
    pub cached_lookup_big_sheet: Option<f64>,

    pub mounting: Option<f64>,
}

impl BenchmarkResults {
    pub fn step(&mut self) -> bool {
        if self.parse_simple.is_none() {
            self.parse_simple = Some(benchmarks::bench_parse_simple());
            true
        } else if self.macro_simple.is_none() {
            self.macro_simple = Some(benchmarks::bench_macro_simple());
            true
        } else if self.macro_inline_simple.is_none() {
            self.macro_inline_simple = Some(benchmarks::bench_macro_inline_simple());
            true
        } else if self.parse_simple_no_cache.is_none() {
            self.parse_simple_no_cache = Some(benchmarks::bench_parse_simple_no_cache());
            true
        } else if self.parse_complex.is_none() {
            self.parse_complex = Some(benchmarks::bench_parse_complex());
            true
        } else if self.macro_complex.is_none() {
            self.macro_complex = Some(benchmarks::bench_macro_complex());
            true
        } else if self.macro_inline_complex.is_none() {
            self.macro_inline_complex = Some(benchmarks::bench_macro_inline_complex());
            true
        } else if self.parse_complex_no_cache.is_none() {
            self.parse_complex_no_cache = Some(benchmarks::bench_parse_complex_no_cache());
            true
        } else if self.cached_lookup.is_none() {
            self.cached_lookup = Some(benchmarks::bench_cached_lookup());
            true
        } else if self.cached_lookup_big_sheet.is_none() {
            self.cached_lookup_big_sheet = Some(benchmarks::bench_cached_lookup_big_sheet());
            true
        } else if self.mounting.is_none() {
            self.mounting = Some(benchmarks::bench_mounting());
            true
        } else {
            false
        }
    }
}
