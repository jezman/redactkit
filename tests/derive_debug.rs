#![cfg(feature = "derive")]

use redactkit::RedactDebug;

#[derive(RedactDebug)]
#[allow(dead_code)]
struct Probe {
    username: String,
}

#[test]
fn derive_generates_debug_impl() {
    let probe = Probe {
        username: "anna".to_string(),
    };

    let debug = format!("{probe:?}");

    assert!(debug.contains("Probe"));
    assert!(!debug.contains("anna"));
}
