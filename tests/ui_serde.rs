#![cfg(feature = "serde")]

#[test]
fn ui_serde() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui_serde/*.rs");
}
