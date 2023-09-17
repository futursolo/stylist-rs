#[allow(dead_code)]
#[rustversion::attr(stable(1.64.0), test)]
fn test_macro_integrations() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/inline_integrations/*-fail.rs");
    t.compile_fail("tests/literal_integrations/*-fail.rs");
    t.pass("tests/sc_integrations/*-pass.rs");
    t.compile_fail("tests/sc_integrations/*-fail.rs");
}
