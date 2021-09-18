#[test]
fn test_macro_integrations() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/inline_integrations/*-fail.rs");
    t.compile_fail("tests/literal_integrations/*-fail.rs");
    t.compile_fail("tests/sc_integrations/*-fail.rs");
}
